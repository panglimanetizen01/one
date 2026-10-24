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
use std::process::Command;

const HEADER: &str = "ONE_PACKAGE_PROVENANCE_V1";
const HEADER_V2: &str = "ONE_PACKAGE_PROVENANCE_V2";
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

struct V2ProvenanceFields<'a> {
    publisher: &'a str,
    artifact_digest: &'a str,
    source_digest: &'a str,
    source_revision: &'a str,
    one_version: &'a str,
    builder_os: &'a str,
    builder_arch: &'a str,
    rustc_version: &'a str,
    cargo_version: &'a str,
    dependency_model: &'a str,
}

fn signed_message_v2(fields: &V2ProvenanceFields<'_>) -> String {
    format!(
        "{HEADER_V2}\npublisher={}\nartifact_sha256={}\nsource_sha256={}\nsource_revision={}\none_version={}\nbuilder_os={}\nbuilder_arch={}\nrustc_version={}\ncargo_version={}\ndependency_model={}\n",
        fields.publisher,
        fields.artifact_digest,
        fields.source_digest,
        fields.source_revision,
        fields.one_version,
        fields.builder_os,
        fields.builder_arch,
        fields.rustc_version,
        fields.cargo_version,
        fields.dependency_model,
    )
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot run git for build provenance: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git command failed while preparing build provenance: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output.stdout)
}

fn validate_empty_dependency_manifest(contents: &str) -> Result<(), String> {
    let mut seen = BTreeMap::new();
    for (index, raw_line) in contents.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((raw_key, raw_value)) = line.split_once('=') else {
            return Err(format!("invalid package manifest line {}", index + 1));
        };
        let key = raw_key.trim();
        let value = raw_value.trim();
        if !matches!(key, "nama" | "versi") || seen.insert(key, value).is_some() {
            return Err(
                "v0.1 build provenance requires the normative empty-dependency manifest; dependency declarations are unsupported"
                    .to_owned(),
            );
        }
        if value.strip_prefix('"').and_then(|value| value.strip_suffix('"')).is_none_or(str::is_empty) {
            return Err(format!("invalid package manifest value on line {}", index + 1));
        }
    }
    if !seen.contains_key("nama") || !seen.contains_key("versi") {
        return Err("package manifest must declare nama and versi".to_owned());
    }
    Ok(())
}

fn collect_source_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<(String, PathBuf)>,
    total_bytes: &mut u64,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read source directory {}: {error}", directory.display()))?
        .map(|entry| entry.map(|entry| entry.path()).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    for path in entries {
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot stat source input {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(format!("source provenance refuses symlinks: {}", path.display()));
        }
        if metadata.is_dir() {
            collect_source_files(root, &path, files, total_bytes)?;
            continue;
        }
        if !metadata.is_file() {
            return Err(format!("source provenance requires regular files: {}", path.display()));
        }
        if metadata.len() > 16 * 1024 * 1024 {
            return Err(format!("source file exceeds 16 MiB: {}", path.display()));
        }
        *total_bytes = total_bytes.saturating_add(metadata.len());
        if *total_bytes > 64 * 1024 * 1024 {
            return Err("source tree exceeds 64 MiB".to_owned());
        }
        let relative = path.strip_prefix(root).map_err(|_| "source file escaped source root".to_owned())?;
        let relative_name = relative
            .components()
            .map(|component| component.as_os_str().to_str().ok_or_else(|| "source path is not valid UTF-8".to_owned()))
            .collect::<Result<Vec<_>, _>>()?
            .join("/");
        files.push((relative_name, path));
        if files.len() > 10_000 {
            return Err("source tree exceeds 10000 files".to_owned());
        }
    }
    Ok(())
}

/// Return a deterministic source-tree digest and the clean Git commit that identifies it.
/// Only tracked, clean project inputs are accepted; v0.1 manifests intentionally support no
/// external dependency declarations.
fn source_snapshot(root: &Path) -> Result<(String, String), String> {
    let root = fs::canonicalize(root).map_err(|error| format!("cannot resolve source root: {error}"))?;
    if !root.is_dir() {
        return Err("source root must be a directory".to_owned());
    }
    let manifest = root.join("one.toml");
    let manifest_metadata = fs::symlink_metadata(&manifest)
        .map_err(|error| format!("cannot stat source manifest: {error}"))?;
    if manifest_metadata.file_type().is_symlink() || !manifest_metadata.is_file() {
        return Err("one.toml must be a regular non-symlink file".to_owned());
    }
    let manifest_bytes = read_bounded(&manifest, 1024 * 1024, "source manifest")?;
    let manifest_text = String::from_utf8(manifest_bytes)
        .map_err(|_| "source manifest is not valid UTF-8".to_owned())?;
    validate_empty_dependency_manifest(&manifest_text)?;

    let source_directory = root.join("src");
    let source_metadata = fs::symlink_metadata(&source_directory)
        .map_err(|error| format!("cannot stat source directory: {error}"))?;
    if source_metadata.file_type().is_symlink() || !source_metadata.is_dir() {
        return Err("src must be a regular non-symlink directory".to_owned());
    }
    if !source_directory.join("main.one").is_file() {
        return Err("source root must contain src/main.one".to_owned());
    }

    let status_args = ["status", "--porcelain=v1", "--untracked-files=all", "--ignored=matching", "--", "one.toml", "src"];
    let status_before = git_output(&root, &status_args)?;
    if !status_before.is_empty() {
        return Err("source inputs are dirty, untracked, or ignored; commit the exact source snapshot before signing".to_owned());
    }
    let tracked = git_output(&root, &["ls-files", "-z", "--", "one.toml", "src"])?;
    let tracked = tracked
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8(path.to_vec()).map_err(|_| "tracked source path is not valid UTF-8".to_owned()))
        .collect::<Result<std::collections::BTreeSet<_>, _>>()?;

    let mut files = vec![("one.toml".to_owned(), manifest.clone())];
    let mut total_bytes = manifest_metadata.len();
    collect_source_files(&root, &source_directory, &mut files, &mut total_bytes)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));
    for (relative, _) in &files {
        if !tracked.contains(relative) {
            return Err(format!("source input is not tracked by Git: {relative}"));
        }
    }

    let revision = String::from_utf8(git_output(&root, &["rev-parse", "--verify", "HEAD"])?)
        .map_err(|_| "Git revision is not valid UTF-8".to_owned())?
        .trim()
        .to_owned();
    if !(40..=64).contains(&revision.len()) || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("Git revision must be a full hexadecimal commit ID".to_owned());
    }

    let mut hasher = Sha256::new();
    for (relative, path) in files {
        let bytes = read_bounded(&path, 16 * 1024 * 1024, "source input")?;
        hasher.update((relative.len() as u64).to_be_bytes());
        hasher.update(relative.as_bytes());
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(&bytes);
    }
    let digest = hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let status_after = git_output(&root, &status_args)?;
    let revision_after = String::from_utf8(git_output(&root, &["rev-parse", "--verify", "HEAD"])?)
        .map_err(|_| "Git revision is not valid UTF-8".to_owned())?
        .trim()
        .to_owned();
    if !status_after.is_empty() || revision_after != revision {
        return Err("source inputs changed while the provenance digest was being calculated".to_owned());
    }
    Ok((digest, revision))
}

fn tool_version(command: &str) -> Result<String, String> {
    let output = Command::new(command)
        .arg("--version")
        .output()
        .map_err(|error| format!("cannot read {command} version for provenance: {error}"))?;
    if !output.status.success() {
        return Err(format!("{command} --version failed while preparing provenance"));
    }
    let version = String::from_utf8(output.stdout)
        .map_err(|_| format!("{command} version is not valid UTF-8"))?
        .trim()
        .to_owned();
    if version.is_empty() || version.len() > 256 || version.contains('\n') || version.contains('\r') {
        return Err(format!("{command} version string is invalid"));
    }
    Ok(version)
}

/// Sign a package and a source-bound build attestation. The source must be a clean,
/// Git-tracked v0.1 project with no dependency declarations.
pub fn sign_build_provenance(
    artifact: &Path,
    publisher: &str,
    private_key_file: &Path,
    source_root: &Path,
) -> Result<PathBuf, String> {
    validate_publisher(publisher)?;
    let metadata = fs::metadata(artifact).map_err(|error| format!("cannot stat package artifact: {error}"))?;
    if !metadata.is_file() {
        return Err("package artifact must be a regular file".into());
    }
    let artifact_bytes = read_bounded(artifact, MAX_PACKAGE_BYTES, "package artifact")?;
    let seed_text = read_bounded_text(private_key_file, MAX_PRIVATE_KEY_BYTES, "publisher key")?;
    let seed = decode_hex::<32>(seed_text.trim(), "private key seed")?;
    let signing_key = SigningKey::from_bytes(&seed);
    let public_key = signing_key.verifying_key().to_bytes();
    let artifact_digest = sha256_hex(&artifact_bytes);
    let (source_digest, source_revision) = source_snapshot(source_root)?;
    let one_version = env!("CARGO_PKG_VERSION");
    let builder_os = std::env::consts::OS;
    let builder_arch = std::env::consts::ARCH;
    let rustc_version = tool_version("rustc")?;
    let cargo_version = tool_version("cargo")?;
    let dependency_model = "empty-v0.1";
    let provenance_fields = V2ProvenanceFields {
        publisher,
        artifact_digest: &artifact_digest,
        source_digest: &source_digest,
        source_revision: &source_revision,
        one_version,
        builder_os,
        builder_arch,
        rustc_version: &rustc_version,
        cargo_version: &cargo_version,
        dependency_model,
    };
    let message = signed_message_v2(&provenance_fields);
    let signature = signing_key.sign(message.as_bytes());
    let contents = format!(
        "{HEADER_V2}\npublisher={publisher}\npublic_key={}\nartifact_sha256={artifact_digest}\nsource_sha256={source_digest}\nsource_revision={source_revision}\none_version={one_version}\nbuilder_os={builder_os}\nbuilder_arch={builder_arch}\nrustc_version={rustc_version}\ncargo_version={cargo_version}\ndependency_model={dependency_model}\nsignature={}\n",
        hex(&public_key),
        hex(&signature.to_bytes()),
    );
    let sidecar = sidecar_path(artifact);
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&sidecar)
        .map_err(|error| format!("cannot create provenance sidecar without overwriting an existing path: {error}"))?;
    if let Err(error) = output.write_all(contents.as_bytes()).and_then(|()| output.sync_all()) {
        drop(output);
        let _ = fs::remove_file(&sidecar);
        return Err(format!("cannot persist provenance sidecar: {error}"));
    }
    Ok(sidecar)
}

/// Sign the exact package bytes with a 32-byte Ed25519 seed stored as 64 hex characters.
/// The generated sidecar is not trusted until its publisher key appears in a verifier's trust store.
#[cfg(test)]
fn sign_artifact(artifact: &Path, publisher: &str, private_key_file: &Path) -> Result<PathBuf, String> {
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
fn parse_sidecar(contents: &str) -> Result<(&str, BTreeMap<&str, &str>), String> {
    let mut lines = contents.lines();
    let header = lines.next().ok_or_else(|| "empty provenance sidecar".to_owned())?;
    if header != HEADER && header != HEADER_V2 {
        return Err("unsupported package provenance format".into());
    }
    let mut fields = BTreeMap::new();
    for line in lines {
        let Some((key, value)) = line.split_once('=') else {
            return Err("malformed provenance field".into());
        };
        let allowed = if header == HEADER {
            matches!(key, "publisher" | "public_key" | "artifact_sha256" | "signature")
        } else {
            matches!(
                key,
                "publisher" | "public_key" | "artifact_sha256" | "source_sha256"
                    | "source_revision" | "one_version" | "builder_os" | "builder_arch"
                    | "rustc_version" | "cargo_version" | "dependency_model" | "signature"
            )
        };
        if !allowed || fields.insert(key, value).is_some() {
            return Err("unknown or duplicate provenance field".into());
        }
    }
    let expected = if header == HEADER { 4 } else { 12 };
    if fields.len() != expected {
        return Err("provenance sidecar is missing required fields".into());
    }
    Ok((header, fields))
}

/// Verify content integrity, the Ed25519 signature, and the publisher-to-key trust binding.
/// Trust store format: one publisher_id=64_hex_character_public_key entry per line.
pub fn verify_artifact(artifact: &Path, trust_store: &Path) -> Result<String, String> {
    let bytes = read_bounded(artifact, MAX_PACKAGE_BYTES, "package artifact")?;
    let sidecar = sidecar_path(artifact);
    let contents = read_bounded_text(&sidecar, MAX_SIDECAR_BYTES, &format!("provenance sidecar {}", sidecar.display()))?;
    let (header, fields) = parse_sidecar(&contents)?;

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

    let message = if header == HEADER {
        signed_message(publisher, digest)
    } else {
        let source_digest = fields["source_sha256"];
        if source_digest.len() != 64 || source_digest.bytes().any(|byte| nibble(byte).is_none()) {
            return Err("source digest must be 64 hexadecimal characters".into());
        }
        let source_revision = fields["source_revision"];
        if !(40..=64).contains(&source_revision.len())
            || !source_revision.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("source revision must be a full hexadecimal commit ID".into());
        }
        if fields["dependency_model"] != "empty-v0.1" {
            return Err("unsupported dependency model in build provenance".into());
        }
        for field in ["one_version", "builder_os", "builder_arch", "rustc_version", "cargo_version"] {
            if fields[field].is_empty() || fields[field].contains(['\n', '\r']) {
                return Err(format!("invalid build provenance field: {field}"));
            }
        }
        let provenance_fields = V2ProvenanceFields {
            publisher,
            artifact_digest: digest,
            source_digest,
            source_revision,
            one_version: fields["one_version"],
            builder_os: fields["builder_os"],
            builder_arch: fields["builder_arch"],
            rustc_version: fields["rustc_version"],
            cargo_version: fields["cargo_version"],
            dependency_model: fields["dependency_model"],
        };
        signed_message_v2(&provenance_fields)
    };

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
    verifying_key.verify(message.as_bytes(), &signature).map_err(|_| "Ed25519 provenance signature verification failed".to_owned())?;
    Ok(publisher.to_owned())
}

/// Verify the signed attestation against the caller's exact, clean source checkout.
pub fn verify_build_provenance(
    artifact: &Path,
    trust_store: &Path,
    source_root: &Path,
) -> Result<String, String> {
    let publisher = verify_artifact(artifact, trust_store)?;
    let sidecar = sidecar_path(artifact);
    let contents = read_bounded_text(&sidecar, MAX_SIDECAR_BYTES, "provenance sidecar")?;
    let (header, fields) = parse_sidecar(&contents)?;
    if header != HEADER_V2 {
        return Err("artifact-only V1 signatures do not contain source-bound build provenance".to_owned());
    }
    let (source_digest, source_revision) = source_snapshot(source_root)?;
    if fields["source_sha256"] != source_digest {
        return Err("source tree digest does not match signed build provenance".to_owned());
    }
    if fields["source_revision"] != source_revision {
        return Err("source Git revision does not match signed build provenance".to_owned());
    }
    Ok(publisher)
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

    fn init_git_source(root: &Path) {
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("one.toml"), "nama = \"demo\"\nversi = \"0.1.0\"\n").unwrap();
        fs::write(root.join("src/main.one"), "Buat program demo\ntampilkan \"halo\"\n").unwrap();
        for args in [
            vec!["init"],
            vec!["config", "user.name", "ONE Provenance Test"],
            vec!["config", "user.email", "one-provenance-test@example.invalid"],
            vec!["config", "core.autocrlf", "false"],
            vec!["add", "--", "one.toml", "src/main.one"],
            vec!["commit", "-m", "source fixture"],
        ] {
            let output = Command::new("git").arg("-C").arg(root).args(args).output().unwrap();
            assert!(output.status.success(), "git fixture setup failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    #[test]
    fn source_bound_attestation_authenticates_artifact_and_clean_source_snapshot() {
        let root = std::env::temp_dir().join(format!(
            "one-package-build-attestation-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        let source_root = root.join("project");
        fs::create_dir_all(&root).unwrap();
        init_git_source(&source_root);

        let artifact = root.join("demo.onepkg");
        let key_file = root.join("publisher.seed");
        let trust_file = root.join("trusted-publishers.txt");
        fs::write(&artifact, b"One package v0.1\nartifact bytes").unwrap();
        let seed = [17u8; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        fs::write(&key_file, hex(&seed)).unwrap();
        fs::write(&trust_file, format!("example-publisher={}\n", hex(&signing_key.verifying_key().to_bytes()))).unwrap();

        sign_build_provenance(&artifact, "example-publisher", &key_file, &source_root).unwrap();
        assert_eq!(verify_artifact(&artifact, &trust_file).unwrap(), "example-publisher");
        assert_eq!(
            verify_build_provenance(&artifact, &trust_file, &source_root).unwrap(),
            "example-publisher"
        );

        let sidecar = sidecar_path(&artifact);
        let original_sidecar = fs::read_to_string(&sidecar).unwrap();
        let tampered = original_sidecar.replace("builder_arch=", "builder_arch=tampered_");
        fs::write(&sidecar, tampered).unwrap();
        assert!(verify_artifact(&artifact, &trust_file).unwrap_err().contains("signature"));
        fs::write(&sidecar, original_sidecar).unwrap();

        fs::write(source_root.join("src/main.one"), "Buat program demo\ntampilkan \"changed\"\n").unwrap();
        let output = Command::new("git").arg("-C").arg(&source_root).args(["add", "--", "src/main.one"]).output().unwrap();
        assert!(output.status.success());
        let output = Command::new("git").arg("-C").arg(&source_root).args(["commit", "-m", "change source"]).output().unwrap();
        assert!(output.status.success());
        assert!(verify_build_provenance(&artifact, &trust_file, &source_root).unwrap_err().contains("source tree digest"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_attestation_rejects_dependency_declarations_outside_v01_contract() {
        let root = std::env::temp_dir().join(format!(
            "one-package-dependency-contract-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("one.toml"),
            "nama = \"demo\"\nversi = \"0.1.0\"\ndependencies = \"example\"\n",
        )
        .unwrap();
        fs::write(root.join("src/main.one"), "Buat program demo\n").unwrap();
        assert!(source_snapshot(&root).unwrap_err().contains("empty-dependency"));
        let _ = fs::remove_dir_all(root);
    }


}
