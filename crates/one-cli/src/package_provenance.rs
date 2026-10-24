//! Ed25519-signed package provenance sidecars.
//!
//! A SHA-256 digest detects content changes; the Ed25519 signature and a separately
//! configured publisher trust store authenticate which publisher approved the bytes.
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const HEADER: &str = "ONE_PACKAGE_PROVENANCE_V1";
const MAX_PACKAGE_BYTES: usize = 64 * 1024 * 1024;
const MAX_PRIVATE_KEY_BYTES: usize = 256;
const MAX_SIDECAR_BYTES: usize = 16 * 1024;
const MAX_TRUST_STORE_BYTES: usize = 1024 * 1024;

fn read_bounded(path: &Path, limit: usize, label: &str) -> Result<Vec<u8>, String> {
    let file = File::open(path).map_err(|error| format!("cannot open {label}: {error}"))?;
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read {label}: {error}"))?;
    if bytes.len() > limit {
        return Err(format!("{label} exceeds the {}-byte resource limit", limit));
    }
    Ok(bytes)
}

fn read_bounded_text(path: &Path, limit: usize, label: &str) -> Result<String, String> {
    let bytes = read_bounded(path, limit, label)?;
    String::from_utf8(bytes).map_err(|_| format!("{label} is not valid UTF-8"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn decode_hex<const N: usize>(input: &str, field: &str) -> Result<[u8; N], String> {
    if input.len() != N * 2 {
        return Err(format!("{field} must contain exactly {} hexadecimal characters", N * 2));
    }
    let bytes = input.as_bytes();
    let mut decoded = [0u8; N];
    for index in 0..N {
        let high = nibble(bytes[index * 2]).ok_or_else(|| format!("{field} contains non-hexadecimal data"))?;
        let low = nibble(bytes[index * 2 + 1]).ok_or_else(|| format!("{field} contains non-hexadecimal data"))?;
        decoded[index] = (high << 4) | low;
    }
    Ok(decoded)
}

fn nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn validate_publisher(publisher: &str) -> Result<(), String> {
    if publisher.is_empty()
        || publisher.len() > 128
        || !publisher.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err("publisher ID must be 1-128 ASCII letters, digits, dots, underscores, or hyphens".into());
    }
    Ok(())
}

fn sidecar_path(artifact: &Path) -> PathBuf {
    let mut name = artifact.as_os_str().to_os_string();
    name.push(".provenance");
    PathBuf::from(name)
}

fn signed_message(publisher: &str, digest: &str) -> String {
    format!("{HEADER}\npublisher={publisher}\nartifact_sha256={digest}\n")
}

/// Sign the exact package bytes with a 32-byte Ed25519 seed stored as 64 hex characters.
/// The generated sidecar is not trusted until its publisher key appears in a verifier's trust store.
pub fn sign_artifact(artifact: &Path, publisher: &str, private_key_file: &Path) -> Result<PathBuf, String> {
    validate_publisher(publisher)?;
    let metadata = fs::metadata(artifact).map_err(|error| format!("cannot stat package artifact: {error}"))?;
    if !metadata.is_file() {
        return Err("package artifact must be a regular file".into());
    }
    let bytes = read_bounded(artifact, MAX_PACKAGE_BYTES, "package artifact")?;
    let seed_text = read_bounded_text(private_key_file, MAX_PRIVATE_KEY_BYTES, "publisher key")?;
    let seed = decode_hex::<32>(seed_text.trim(), "private key seed")?;
    let signing_key = SigningKey::from_bytes(&seed);
    let public_key = signing_key.verifying_key().to_bytes();
    let digest = sha256_hex(&bytes);
    let message = signed_message(publisher, &digest);
    let signature = signing_key.sign(message.as_bytes());
    let sidecar = sidecar_path(artifact);
    let contents = format!(
        "{HEADER}\npublisher={publisher}\npublic_key={}\nartifact_sha256={digest}\nsignature={}\n",
        hex(&public_key),
        hex(&signature.to_bytes()),
    );
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&sidecar)
        .map_err(|error| format!("cannot create provenance sidecar without overwriting an existing path: {error}"))?;
    if let Err(error) = output.write_all(contents.as_bytes()) {
        drop(output);
        let _ = fs::remove_file(&sidecar);
        return Err(format!("cannot write provenance sidecar: {error}"));
    }
    output.sync_all().map_err(|error| format!("cannot sync provenance sidecar: {error}"))?;
    Ok(sidecar)
}

/// Verify content integrity, the Ed25519 signature, and the publisher-to-key trust binding.
/// Trust store format: one publisher_id=64_hex_character_public_key entry per line.
pub fn verify_artifact(artifact: &Path, trust_store: &Path) -> Result<String, String> {
    let bytes = read_bounded(artifact, MAX_PACKAGE_BYTES, "package artifact")?;
    let sidecar = sidecar_path(artifact);
    let contents = read_bounded_text(&sidecar, MAX_SIDECAR_BYTES, &format!("provenance sidecar {}", sidecar.display()))?;
    let mut lines = contents.lines();
    if lines.next() != Some(HEADER) {
        return Err("unsupported package provenance format".into());
    }
    let mut fields = BTreeMap::new();
    for line in lines {
        let Some((key, value)) = line.split_once('=') else {
            return Err("malformed provenance field".into());
        };
        if !matches!(key, "publisher" | "public_key" | "artifact_sha256" | "signature")
            || fields.insert(key, value).is_some()
        {
            return Err("unknown or duplicate provenance field".into());
        }
    }
    if fields.len() != 4 {
        return Err("provenance sidecar is missing required fields".into());
    }
    let publisher = *fields.get("publisher").ok_or("missing publisher")?;
    validate_publisher(publisher)?;
    let public_key_bytes = decode_hex::<32>(fields["public_key"], "public key")?;
    let signature_bytes = decode_hex::<64>(fields["signature"], "signature")?;
    let digest = fields["artifact_sha256"];
    if digest.len() != 64 || digest.bytes().any(|byte| nibble(byte).is_none()) {
        return Err("artifact digest must be 64 hexadecimal characters".into());
    }
    if sha256_hex(&bytes) != digest.to_ascii_lowercase() {
        return Err("package artifact SHA-256 does not match its provenance sidecar".into());
    }

    let trust_text = read_bounded_text(trust_store, MAX_TRUST_STORE_BYTES, "publisher trust store")?;
    let mut trusted = BTreeMap::new();
    for (index, raw_line) in trust_text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((id, key)) = line.split_once('=') else {
            return Err(format!("malformed trust-store entry on line {}", index + 1));
        };
        let id = id.trim();
        validate_publisher(id)?;
        let key = decode_hex::<32>(key.trim(), "trusted public key")?;
        if trusted.insert(id.to_owned(), key).is_some() {
            return Err(format!("duplicate publisher '{}' in trust store", id));
        }
    }
    let trusted_key = trusted.get(publisher).ok_or_else(|| format!("publisher '{}' is not trusted", publisher))?;
    if trusted_key != &public_key_bytes {
        return Err(format!("public key in provenance does not match trusted publisher '{}'", publisher));
    }
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes).map_err(|_| "invalid Ed25519 public key".to_owned())?;
    let signature = Signature::from_bytes(&signature_bytes);
    let message = signed_message(publisher, digest);
    verifying_key.verify(message.as_bytes(), &signature).map_err(|_| "Ed25519 provenance signature verification failed".to_owned())?;
    Ok(publisher.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn bounded_reads_reject_oversized_inputs() {
        let root = std::env::temp_dir().join(format!(
            "one-package-bounded-read-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let input = root.join("oversized.bin");
        fs::write(&input, b"12345").unwrap();
        let error = read_bounded(&input, 4, "test input").unwrap_err();
        assert!(error.contains("4-byte resource limit"), "{error}");
        assert_eq!(read_bounded(&input, 5, "test input").unwrap(), b"12345");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn signed_package_provenance_requires_trusted_key_and_unchanged_content() {
        let root = std::env::temp_dir().join(format!(
            "one-package-provenance-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let artifact = root.join("demo.onepkg");
        let key_file = root.join("publisher.seed");
        let trust_file = root.join("trusted-publishers.txt");
        fs::write(&artifact, b"One package v0.1\nartifact bytes").unwrap();

        let seed = [7u8; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        fs::write(&key_file, hex(&seed)).unwrap();
        fs::write(&trust_file, format!("example-publisher={}\n", hex(&signing_key.verifying_key().to_bytes()))).unwrap();

        sign_artifact(&artifact, "example-publisher", &key_file).unwrap();
        assert_eq!(verify_artifact(&artifact, &trust_file).unwrap(), "example-publisher");

        fs::write(&artifact, b"tampered package bytes").unwrap();
        assert!(verify_artifact(&artifact, &trust_file).unwrap_err().contains("SHA-256"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn signed_package_provenance_rejects_untrusted_publishers() {
        let root = std::env::temp_dir().join(format!(
            "one-package-untrusted-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let artifact = root.join("demo.onepkg");
        let key_file = root.join("publisher.seed");
        let trust_file = root.join("trusted-publishers.txt");
        fs::write(&artifact, b"One package v0.1\nartifact bytes").unwrap();
        let seed = [9u8; 32];
        fs::write(&key_file, hex(&seed)).unwrap();
        fs::write(&trust_file, "somebody-else=0000000000000000000000000000000000000000000000000000000000000000\n").unwrap();

        sign_artifact(&artifact, "example-publisher", &key_file).unwrap();
        assert!(verify_artifact(&artifact, &trust_file).unwrap_err().contains("not trusted"));
        let _ = fs::remove_dir_all(root);
    }
}
