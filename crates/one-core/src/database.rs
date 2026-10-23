use crate::adapter::{AdapterArgument, AdapterError, AdapterMetadata, AdapterRequest, AdapterResult, PlatformKind, ToolAdapter};
use crate::capability::Capability;
use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, ExecutionScope};
use rusqlite::{types::ValueRef, Connection, OpenFlags};

const MAX_DATABASE_OUTPUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_DATABASE_QUERY_BYTES: usize = 1024 * 1024;
const MAX_DATABASE_PARAMETER_BYTES: usize = 1024 * 1024;
const MAX_DATABASE_TOTAL_PARAMETER_BYTES: usize = 16 * 1024 * 1024;
const MAX_DATABASE_PARAMETERS: usize = 256;
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
                if sql.len() > MAX_DATABASE_QUERY_BYTES {
                    return Err(AdapterError::ResourceLimitExceeded);
                }
                if rest.len() > MAX_DATABASE_PARAMETERS {
                    return Err(AdapterError::ResourceLimitExceeded);
                }
                if !sql.trim_start().to_ascii_uppercase().starts_with("SELECT") {
                    return Err(AdapterError::InvalidArguments);
                }
                let mut values = Vec::with_capacity(rest.len());
                let mut total_parameter_bytes = 0usize;
                for argument in rest {
                    match argument {
                        AdapterArgument::Text(value) => {
                            if value.len() > MAX_DATABASE_PARAMETER_BYTES {
                                return Err(AdapterError::ResourceLimitExceeded);
                            }
                            total_parameter_bytes = total_parameter_bytes
                                .checked_add(value.len())
                                .ok_or(AdapterError::ResourceLimitExceeded)?;
                            if total_parameter_bytes > MAX_DATABASE_TOTAL_PARAMETER_BYTES {
                                return Err(AdapterError::ResourceLimitExceeded);
                            }
                            values.push(value.clone());
                        }
                        _ => return Err(AdapterError::InvalidArguments),
                    }
                }
                (sql.as_str(), values)
            }
            _ => return Err(AdapterError::InvalidArguments),
        };

        // DatabaseQuery is a read-only effect. Keep in-memory queries available for
        // deterministic tests, but never create or open a file-backed database writable.
        let connection = if path == ":memory:" {
            Connection::open_in_memory()
        } else {
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        }
        .map_err(|_| AdapterError::BackendFailure)?;
        connection
            .pragma_update(None, "query_only", true)
            .map_err(|_| AdapterError::BackendFailure)?;
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

    fn temporary_database_path(label: &str) -> std::path::PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("one-db-{label}-{}-{nonce}.sqlite", std::process::id()))
    }

    fn select_request(path: &std::path::Path, sql: String) -> AdapterRequest {
        AdapterRequest::new(CapabilityRequest::new(
            crate::orchestration::SemanticCapabilityId::DatabaseQuery,
            Capability::DatabaseQuery,
            ExecutionScope::resource(path.to_string_lossy()),
            crate::orchestration::EffectClass::Read,
        ))
        .with_argument(AdapterArgument::Text(sql))
    }

    #[test]
    fn sqlite_adapter_rejects_sql_over_query_byte_limit() {
        let sql = format!("SELECT {}", " ".repeat(MAX_DATABASE_QUERY_BYTES));
        let adapter = SqliteAdapter::new();
        let result = adapter.invoke(&select_request(std::path::Path::new(":memory:"), sql));
        assert_eq!(result, Err(AdapterError::ResourceLimitExceeded));
    }

    #[test]
    fn sqlite_adapter_rejects_individually_oversized_parameters() {
        let adapter = SqliteAdapter::new();
        let result = adapter.invoke(
            &select_request(std::path::Path::new(":memory:"), "SELECT ?1".into())
                .with_argument(AdapterArgument::Text("x".repeat(MAX_DATABASE_PARAMETER_BYTES + 1))),
        );
        assert_eq!(result, Err(AdapterError::ResourceLimitExceeded));
    }

    #[test]
    fn sqlite_adapter_rejects_excessive_parameter_count() {
        let mut request = select_request(std::path::Path::new(":memory:"), "SELECT ?1".into());
        for _ in 0..=MAX_DATABASE_PARAMETERS {
            request = request.with_argument(AdapterArgument::Text("x".into()));
        }
        let result = SqliteAdapter::new().invoke(&request);
        assert_eq!(result, Err(AdapterError::ResourceLimitExceeded));
    }

    #[test]
    fn sqlite_adapter_rejects_aggregate_parameter_bytes_over_the_limit() {
        let mut request = select_request(std::path::Path::new(":memory:"), "SELECT ?1".into());
        for _ in 0..=(MAX_DATABASE_TOTAL_PARAMETER_BYTES / MAX_DATABASE_PARAMETER_BYTES) {
            request = request.with_argument(AdapterArgument::Text("x".repeat(MAX_DATABASE_PARAMETER_BYTES)));
        }
        let result = SqliteAdapter::new().invoke(&request);
        assert_eq!(result, Err(AdapterError::ResourceLimitExceeded));
    }

    #[test]
    fn sqlite_adapter_rejects_query_results_over_the_output_byte_limit() {
        let path = temporary_database_path("output-limit");
        let connection = Connection::open(&path).expect("database");
        connection
            .execute_batch(&format!(
                "CREATE TABLE data (value BLOB); INSERT INTO data VALUES (zeroblob({}));",
                MAX_DATABASE_OUTPUT_BYTES + 1
            ))
            .expect("large blob");
        drop(connection);

        let adapter = SqliteAdapter::new();
        let result = adapter.invoke(&select_request(&path, "SELECT value FROM data".into()));
        assert_eq!(result, Err(AdapterError::ResourceLimitExceeded));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn sqlite_adapter_rejects_query_results_over_the_row_limit() {
        let path = temporary_database_path("row-limit");
        drop(Connection::open(&path).expect("database"));
        let sql = format!(
            "SELECT '' FROM (WITH RECURSIVE rows(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM rows WHERE x < {}) SELECT x FROM rows)",
            MAX_DATABASE_ROWS + 1
        );
        let adapter = SqliteAdapter::new();
        let result = adapter.invoke(&select_request(&path, sql));
        assert_eq!(result, Err(AdapterError::ResourceLimitExceeded));
        let _ = std::fs::remove_file(path);
    }

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
