use crate::adapter::{AdapterArgument, AdapterError, AdapterMetadata, AdapterRequest, AdapterResult, PlatformKind, ToolAdapter};
use crate::capability::Capability;
use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, ExecutionScope};
use rusqlite::{types::ValueRef, Connection};

const MAX_DATABASE_OUTPUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_DATABASE_ROWS: usize = 100_000;

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

    fn value_text(value: ValueRef<'_>) -> Result<String, AdapterError> {
        match value {
            ValueRef::Null => Ok("null".into()),
            ValueRef::Integer(value) => Ok(value.to_string()),
            ValueRef::Real(value) => Ok(value.to_string()),
            ValueRef::Text(value) => {
                if value.len() > MAX_DATABASE_OUTPUT_BYTES {
                    return Err(AdapterError::ResourceLimitExceeded);
                }
                Ok(String::from_utf8_lossy(value).into_owned())
            }
            ValueRef::Blob(value) => {
                let capacity = value.len().checked_mul(2).ok_or(AdapterError::ResourceLimitExceeded)?;
                if capacity > MAX_DATABASE_OUTPUT_BYTES {
                    return Err(AdapterError::ResourceLimitExceeded);
                }
                let mut output = String::with_capacity(capacity);
                for byte in value {
                    use std::fmt::Write as _;
                    write!(&mut output, "{byte:02x}").map_err(|_| AdapterError::BackendFailure)?;
                }
                Ok(output)
            }
        }
    }

    fn append_bounded(output: &mut String, fragment: &str) -> Result<(), AdapterError> {
        let length = output.len().checked_add(fragment.len()).ok_or(AdapterError::ResourceLimitExceeded)?;
        if length > MAX_DATABASE_OUTPUT_BYTES {
            return Err(AdapterError::ResourceLimitExceeded);
        }
        output.push_str(fragment);
        Ok(())
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

        let connection = Connection::open(path).map_err(|_| AdapterError::BackendFailure)?;
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
        let mut row_count = 0usize;
        while let Some(row) = rows.next().map_err(|_| AdapterError::BackendFailure)? {
            row_count = row_count.checked_add(1).ok_or(AdapterError::ResourceLimitExceeded)?;
            if row_count > MAX_DATABASE_ROWS {
                return Err(AdapterError::ResourceLimitExceeded);
            }
            for index in 0..column_count {
                if index > 0 {
                    Self::append_bounded(&mut output, "\t")?;
                }
                let value = row
                    .get_ref(index)
                    .map_err(|_| AdapterError::BackendFailure)?;
                let rendered = Self::value_text(value)?;
                Self::append_bounded(&mut output, &rendered)?;
            }
            Self::append_bounded(&mut output, "\n")?;
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
