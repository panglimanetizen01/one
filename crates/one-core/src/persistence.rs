//! Small versioned, deterministic key/value persistence primitive for language runtimes.
//! The format is intentionally dependency-free and rejects corruption rather than
//! silently returning an empty store.
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const MAGIC: &str = "ONE-KV-1";
const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_ENTRIES: usize = 100_000;
const MAX_FIELD_BYTES: usize = 1024 * 1024;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
    Corrupt { line: usize, reason: String },
    Limit(&'static str),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "kesalahan I/O penyimpanan: {error}"),
            Self::Corrupt { line, reason } => write!(f, "penyimpanan rusak pada baris {line}: {reason}"),
            Self::Limit(reason) => write!(f, "batas penyimpanan terlampaui: {reason}"),
        }
    }
}
impl std::error::Error for StoreError {}
impl From<std::io::Error> for StoreError {
    fn from(value: std::io::Error) -> Self { Self::Io(value) }
}

#[derive(Debug, Clone)]
pub struct KeyValueStore {
    path: PathBuf,
    values: BTreeMap<String, String>,
}

impl KeyValueStore {
    /// Open an existing store or create a new empty one. Existing malformed files fail closed.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let values = if path.exists() {
            let metadata = fs::metadata(&path)?;
            if metadata.len() > MAX_FILE_BYTES { return Err(StoreError::Limit("berkas lebih besar dari 16 MiB")); }
            decode(&fs::read_to_string(&path)?)?
        } else {
            BTreeMap::new()
        };
        Ok(Self { path, values })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values.iter().map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Replace the complete logical contents with one durable write.
    pub fn replace_all(&mut self, values: BTreeMap<String, String>) -> Result<(), StoreError> {
        if values.len() > MAX_ENTRIES { return Err(StoreError::Limit("jumlah entri melebihi 100000")); }
        for (key, value) in &values {
            validate_field(key, "kunci")?;
            validate_field(value, "nilai")?;
        }
        let previous = std::mem::replace(&mut self.values, values);
        if let Err(error) = self.flush() {
            self.values = previous;
            return Err(error);
        }
        Ok(())
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<(), StoreError> {
        let key = key.into();
        let value = value.into();
        validate_field(&key, "kunci")?;
        validate_field(&value, "nilai")?;
        if !self.values.contains_key(&key) && self.values.len() >= MAX_ENTRIES {
            return Err(StoreError::Limit("jumlah entri melebihi 100000"));
        }
        let previous = self.values.insert(key.clone(), value);
        if let Err(error) = self.flush() {
            if let Some(previous) = previous { self.values.insert(key, previous); }
            else { self.values.remove(&key); }
            return Err(error);
        }
        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Result<bool, StoreError> {
        let Some(previous) = self.values.remove(key) else { return Ok(false); };
        if let Err(error) = self.flush() {
            self.values.insert(key.to_owned(), previous);
            return Err(error);
        }
        Ok(true)
    }

    pub fn flush(&self) -> Result<(), StoreError> {
        let encoded = encode(&self.values);
        if encoded.len() as u64 > MAX_FILE_BYTES { return Err(StoreError::Limit("berkas hasil lebih besar dari 16 MiB")); }
        if let Some(parent) = self.path.parent()
            && !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temp = self.path.with_extension(format!("one-tmp-{}-{sequence}", std::process::id()));
        let write_result = (|| -> std::io::Result<()> {
            use std::io::Write as _;
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&temp)?;
            file.write_all(encoded.as_bytes())?;
            file.sync_all()
        })();
        if let Err(error) = write_result {
            let _ = fs::remove_file(&temp);
            return Err(StoreError::Io(error));
        }
        if let Err(error) = replace_file(&temp, &self.path) {
            let _ = fs::remove_file(&temp);
            return Err(StoreError::Io(error));
        }
        #[cfg(unix)]
        if let Some(parent) = self.path.parent().filter(|parent| !parent.as_os_str().is_empty())
            && let Ok(directory) = fs::File::open(parent) {
            let _ = directory.sync_all();
        }
        Ok(())
    }
}

fn validate_field(value: &str, label: &'static str) -> Result<(), StoreError> {
    if label == "kunci" && value.is_empty() { return Err(StoreError::Limit("kunci kosong")); }
    if value.len() > MAX_FIELD_BYTES { return Err(StoreError::Limit("kunci/nilai melebihi 1 MiB")); }
    Ok(())
}

fn encode(values: &BTreeMap<String, String>) -> String {
    let mut output = format!("{MAGIC}\n");
    for (key, value) in values {
        output.push_str(&hex_encode(key.as_bytes()));
        output.push('\t');
        output.push_str(&hex_encode(value.as_bytes()));
        output.push('\n');
    }
    output
}

fn decode(source: &str) -> Result<BTreeMap<String, String>, StoreError> {
    let mut lines = source.lines();
    if lines.next() != Some(MAGIC) {
        return Err(StoreError::Corrupt { line: 1, reason: "magic/version tidak dikenal".into() });
    }
    let mut values = BTreeMap::new();
    for (index, line) in lines.enumerate() {
        let line_number = index + 2;
        let Some((key_hex, value_hex)) = line.split_once('\t') else {
            return Err(StoreError::Corrupt { line: line_number, reason: "pemisah tab tidak ditemukan".into() });
        };
        if value_hex.contains('\t') {
            return Err(StoreError::Corrupt { line: line_number, reason: "terlalu banyak kolom".into() });
        }
        let key_bytes = hex_decode(key_hex).map_err(|reason| StoreError::Corrupt { line: line_number, reason })?;
        let value_bytes = hex_decode(value_hex).map_err(|reason| StoreError::Corrupt { line: line_number, reason })?;
        let key = String::from_utf8(key_bytes).map_err(|_| StoreError::Corrupt { line: line_number, reason: "kunci bukan UTF-8".into() })?;
        let value = String::from_utf8(value_bytes).map_err(|_| StoreError::Corrupt { line: line_number, reason: "nilai bukan UTF-8".into() })?;
        validate_field(&key, "kunci").map_err(|error| StoreError::Corrupt { line: line_number, reason: error.to_string() })?;
        validate_field(&value, "nilai").map_err(|error| StoreError::Corrupt { line: line_number, reason: error.to_string() })?;
        if values.insert(key, value).is_some() {
            return Err(StoreError::Corrupt { line: line_number, reason: "kunci duplikat".into() });
        }
        if values.len() > MAX_ENTRIES { return Err(StoreError::Limit("jumlah entri melebihi 100000")); }
    }
    Ok(values)
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes { out.push(HEX[(byte >> 4) as usize] as char); out.push(HEX[(byte & 0x0f) as usize] as char); }
    out
}

fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) { return Err("panjang data heksadesimal ganjil".into()); }
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks(2) {
        let high = (pair[0] as char).to_digit(16).ok_or_else(|| "digit heksadesimal tidak valid".to_owned())?;
        let low = (pair[1] as char).to_digit(16).ok_or_else(|| "digit heksadesimal tidak valid".to_owned())?;
        output.push(((high << 4) | low) as u8);
    }
    Ok(output)
}

#[cfg(windows)]
fn replace_file(temp: &Path, target: &Path) -> std::io::Result<()> {
    // Windows rename does not replace an existing destination. This fallback avoids
    // claiming crash-atomic replacement on Windows; recovery remains fail-closed.
    if target.exists() {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let backup = target.with_extension(format!("one-backup-{}-{sequence}", std::process::id()));
        fs::rename(target, &backup)?;
        if let Err(error) = fs::rename(temp, target) {
            let _ = fs::rename(&backup, target);
            return Err(error);
        }
        let _ = fs::remove_file(backup);
        Ok(())
    } else {
        fs::rename(temp, target)
    }
}
#[cfg(not(windows))]
fn replace_file(temp: &Path, target: &Path) -> std::io::Result<()> { fs::rename(temp, target) }

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("one-kv-{label}-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()))
    }

    #[test]
    fn values_survive_reopen_and_order_is_deterministic() {
        let path = path("restart");
        let mut store = KeyValueStore::open(&path).unwrap();
        store.set("nama", "Budi").unwrap();
        store.set("angka", "42").unwrap();
        drop(store);
        let reopened = KeyValueStore::open(&path).unwrap();
        assert_eq!(reopened.get("nama"), Some("Budi"));
        assert_eq!(reopened.get("angka"), Some("42"));
        let bytes = fs::read_to_string(&path).unwrap();
        assert!(bytes.starts_with("ONE-KV-1\n"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn corrupt_storage_is_reported_instead_of_reset() {
        let path = path("corrupt");
        fs::write(&path, "not-a-store\n").unwrap();
        assert!(matches!(KeyValueStore::open(&path), Err(StoreError::Corrupt { line: 1, .. })));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn duplicate_keys_and_invalid_fields_are_rejected() {
        assert!(matches!(decode("ONE-KV-1\n61\t31\n61\t32\n"), Err(StoreError::Corrupt { .. })));
        let mut store = KeyValueStore::open(path("invalid")).unwrap();
        assert!(store.set("bad\nkey", "value").is_err());
        let _ = fs::remove_file(store.path);
    }

    #[test]
    fn remove_persists_and_missing_remove_is_idempotent() {
        let path = path("remove");
        let mut store = KeyValueStore::open(&path).unwrap();
        store.set("k", "v").unwrap();
        assert!(store.remove("k").unwrap());
        assert!(!store.remove("k").unwrap());
        assert_eq!(KeyValueStore::open(&path).unwrap().get("k"), None);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn failed_write_rolls_back_in_memory_mutation() {
        let blocker = path("blocked-parent");
        fs::write(&blocker, "not a directory").unwrap();
        let store_path = blocker.join("store.db");
        let mut store = KeyValueStore::open(&store_path).unwrap();
        assert!(store.set("k", "v").is_err());
        assert_eq!(store.get("k"), None);
        let _ = fs::remove_file(blocker);
    }

    #[test]
    fn empty_values_and_multiline_values_round_trip() {
        let path = path("multiline");
        let mut store = KeyValueStore::open(&path).unwrap();
        store.set("empty", "").unwrap();
        store.set("multiline", "baris satu\\nbaris dua\\takhir").unwrap();
        let reopened = KeyValueStore::open(&path).unwrap();
        assert_eq!(reopened.get("empty"), Some(""));
        assert_eq!(reopened.get("multiline"), Some("baris satu\\nbaris dua\\takhir"));
        let _ = fs::remove_file(path);
    }

}
