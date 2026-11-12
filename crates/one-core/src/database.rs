use crate::adapter::{AdapterArgument, AdapterError, AdapterMetadata, AdapterRequest, AdapterResult, PlatformKind, ToolAdapter};
use crate::capability::Capability;
use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, ExecutionScope};
use rusqlite::{types::ValueRef, Connection, OpenFlags};

pub struct SqliteAdapter {
    metadata: AdapterMetadata,
}

impl SqliteAdapter {
    pub fn new() -> Self {
        Self {
            metadata: AdapterMetadata::new(
                AdapterId::new("database.sqlite.query"),
                AdapterStatus::Implemented,
                vec![Capability::DatabaseQuery],
                vec![PlatformKind::Any],
            ),
        }
    }

    fn database_path(request: &CapabilityRequest) -> Result<&str, AdapterError> {
        match &request.scope {
            ExecutionScope::Resource(value) if !value.is_empty() => Ok(value),
            _ => Err(AdapterError::InvalidRequest(
                crate::orchestration::OrchestrationValidationError::EmptyScope,
            )),
        }
    }

    fn value_text(value: ValueRef<'_>) -> String {
        match value {
            ValueRef::Null => "null".into(),
            ValueRef::Integer(value) => value.to_string(),
            ValueRef::Real(value) => value.to_string(),
            ValueRef::Text(value) => String::from_utf8_lossy(value).into_owned(),
            ValueRef::Blob(value) => {
                let mut output = String::with_capacity(value.len() * 2);
                for byte in value {
                    output.push_str(&format!("{byte:02x}"));
                }
                output
            }
        }
    }
}

impl Default for SqliteAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolAdapter for SqliteAdapter {
    fn metadata(&self) -> &AdapterMetadata {
        &self.metadata
    }

    fn invoke(&self, request: &AdapterRequest) -> Result<AdapterResult, AdapterError> {
        self.metadata.validate_request(request.capability())?;
        let path = Self::database_path(request.capability())?;
        let (sql, parameters) = match request.arguments() {
            [AdapterArgument::Text(sql), rest @ ..] => {
                if !sql.trim_start().to_ascii_uppercase().starts_with("SELECT") {
                    return Err(AdapterError::InvalidArguments);
                }
                let mut values = Vec::with_capacity(rest.len());
                for argument in rest {
                    match argument {
                        AdapterArgument::Text(value) => values.push(value.clone()),
                        _ => return Err(AdapterError::InvalidArguments),
                    }
                }
                (sql.as_str(), values)
            }
            _ => return Err(AdapterError::InvalidArguments),
        };

        let connection = if path == ":memory:" { Connection::open_in_memory() } else { Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY) }.map_err(|_| AdapterError::BackendFailure)?;
        let mut statement = connection
            .prepare(sql)
            .map_err(|_| AdapterError::BackendFailure)?;
        let column_count = statement.column_count();
        let parameter_refs: Vec<&dyn rusqlite::ToSql> = parameters
            .iter()
            .map(|value| value as &dyn rusqlite::ToSql)
            .collect();
        let mut rows = statement
            .query(rusqlite::params_from_iter(parameter_refs))
            .map_err(|_| AdapterError::BackendFailure)?;
        let mut output = String::new();
        while let Some(row) = rows.next().map_err(|_| AdapterError::BackendFailure)? {
            for index in 0..column_count {
                if index > 0 {
                    output.push('\t');
                }
                let value = row
                    .get_ref(index)
                    .map_err(|_| AdapterError::BackendFailure)?;
                output.push_str(&Self::value_text(value));
            }
            output.push('\n');
        }

        Ok(AdapterResult {
            success: true,
            output: output.into_bytes(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_adapter_rejects_non_select_statements() {
        let adapter = SqliteAdapter::new();
        let request = CapabilityRequest::new(
            crate::orchestration::SemanticCapabilityId::DatabaseQuery,
            Capability::DatabaseQuery,
            ExecutionScope::resource(":memory:"),
            crate::orchestration::EffectClass::Read,
        );
        assert_eq!(
            adapter.invoke(&AdapterRequest::new(request).with_argument(AdapterArgument::Text(
                "CREATE TABLE data (value TEXT)".into(),
            ))),
            Err(AdapterError::InvalidArguments)
        );
    }
}

#[cfg(test)]
mod read_only_tests {
    use super::*;
    use crate::orchestration::{EffectClass, SemanticCapabilityId};

    fn query_request(path: String) -> AdapterRequest {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::DatabaseQuery,
            Capability::DatabaseQuery,
            ExecutionScope::resource(path),
            EffectClass::Read,
        );
        AdapterRequest::new(request).with_argument(AdapterArgument::Text("SELECT 1".into()))
    }

    #[test]
    fn missing_database_is_not_created_by_read_only_query() {
        let path = std::env::temp_dir().join(format!("one-readonly-missing-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        assert!(!path.exists());

        let result = SqliteAdapter::new().invoke(&query_request(path.to_string_lossy().into_owned()));

        assert_eq!(result, Err(AdapterError::BackendFailure));
        assert!(!path.exists(), "a read-only query must not create a database file");
    }

    #[test]
    fn in_memory_database_remains_available_for_read_only_queries() {
        let result = SqliteAdapter::new()
            .invoke(&query_request(":memory:".into()))
            .expect("in-memory query should remain available");
        assert!(result.success);
        assert_eq!(String::from_utf8(result.output).expect("UTF-8 result"), "1\n");
    }
}
