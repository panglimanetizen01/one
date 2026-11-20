use crate::adapter::{AdapterArgument, AdapterError, AdapterMetadata, AdapterRequest, AdapterResult, PlatformKind, ToolAdapter};
use crate::capability::Capability;
use crate::orchestration::{AdapterId, AdapterStatus, CapabilityRequest, ExecutionScope};
use rusqlite::{types::ValueRef, Connection};

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

    const MAX_SQL_BYTES: usize = 64 * 1024;
    const MAX_PARAMETERS: usize = 128;
    const MAX_PARAMETER_BYTES: usize = 1024 * 1024;
    const MAX_QUERY_ROWS: usize = 100_000;
    const MAX_QUERY_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

    fn append_bounded(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), AdapterError> {
        let next_len = output.len().checked_add(bytes.len())
            .filter(|length| *length <= Self::MAX_QUERY_OUTPUT_BYTES)
            .ok_or(AdapterError::BackendFailure)?;
        output.try_reserve_exact(next_len - output.len()).map_err(|_| AdapterError::BackendFailure)?;
        output.extend_from_slice(bytes);
        Ok(())
    }

    fn append_lossy_text(output: &mut Vec<u8>, mut bytes: &[u8]) -> Result<(), AdapterError> {
        while !bytes.is_empty() {
            match std::str::from_utf8(bytes) {
                Ok(valid) => {
                    Self::append_bounded(output, valid.as_bytes())?;
                    break;
                }
                Err(error) => {
                    let valid_len = error.valid_up_to();
                    if valid_len > 0 {
                        Self::append_bounded(output, &bytes[..valid_len])?;
                    }
                    let invalid_len = error.error_len().unwrap_or(bytes.len() - valid_len);
                    Self::append_bounded(output, "�".as_bytes())?;
                    bytes = &bytes[valid_len + invalid_len..];
                }
            }
        }
        Ok(())
    }

    fn append_value(output: &mut Vec<u8>, value: ValueRef<'_>) -> Result<(), AdapterError> {
        match value {
            ValueRef::Null => Self::append_bounded(output, b"null"),
            ValueRef::Integer(value) => Self::append_bounded(output, value.to_string().as_bytes()),
            ValueRef::Real(value) => Self::append_bounded(output, value.to_string().as_bytes()),
            ValueRef::Text(value) => Self::append_lossy_text(output, value),
            ValueRef::Blob(value) => {
                let hex_len = value.len().checked_mul(2)
                    .filter(|length| output.len().checked_add(*length).is_some_and(|total| total <= Self::MAX_QUERY_OUTPUT_BYTES))
                    .ok_or(AdapterError::BackendFailure)?;
                output.try_reserve_exact(hex_len).map_err(|_| AdapterError::BackendFailure)?;
                const HEX: &[u8; 16] = b"0123456789abcdef";
                for byte in value {
                    output.push(HEX[(byte >> 4) as usize]);
                    output.push(HEX[(byte & 0x0f) as usize]);
                }
                Ok(())
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
                if sql.len() > Self::MAX_SQL_BYTES || rest.len() > Self::MAX_PARAMETERS {
                    return Err(AdapterError::InvalidArguments);
                }
                let statement = sql.trim_start();
                if !statement.get(..6).is_some_and(|prefix| prefix.eq_ignore_ascii_case("SELECT")) {
                    return Err(AdapterError::InvalidArguments);
                }
                if statement.get(6..).and_then(|tail| tail.chars().next()).is_some_and(|next| next.is_ascii_alphanumeric() || next == '_') {
                    return Err(AdapterError::InvalidArguments);
                }
                let mut values = Vec::with_capacity(rest.len());
                let mut total_parameter_bytes = 0usize;
                for argument in rest {
                    match argument {
                        AdapterArgument::Text(value) => {
                            total_parameter_bytes = total_parameter_bytes.checked_add(value.len())
                                .filter(|length| *length <= Self::MAX_PARAMETER_BYTES)
                                .ok_or(AdapterError::InvalidArguments)?;
                            values.push(value.clone());
                        }
                        _ => return Err(AdapterError::InvalidArguments),
                    }
                }
                (sql.as_str(), values)
            }
            _ => return Err(AdapterError::InvalidArguments),
        };

        let connection = if path == ":memory:" { Connection::open_in_memory() } else { Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY) }.map_err(|_| AdapterError::BackendFailure)?;
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
        let mut output = Vec::new();
        let mut row_count = 0usize;
        while let Some(row) = rows.next().map_err(|_| AdapterError::BackendFailure)? {
            row_count = row_count.checked_add(1)
                .filter(|count| *count <= Self::MAX_QUERY_ROWS)
                .ok_or(AdapterError::BackendFailure)?;
            for index in 0..column_count {
                if index > 0 {
                    Self::append_bounded(&mut output, b"\\t")?;
                }
                let value = row.get_ref(index).map_err(|_| AdapterError::BackendFailure)?;
                Self::append_value(&mut output, value)?;
            }
            Self::append_bounded(&mut output, b"\\n")?;
        }

        Ok(AdapterResult { success: true, output })
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
