$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
$env:CARGO_NET_OFFLINE = 'true'

Write-Host '=== ONE OFFLINE VERIFICATION ==='
try { Write-Host "commit: $(git rev-parse HEAD)" } catch { Write-Host 'commit: unknown' }
Write-Host "rust: $(rustc --version)"
Write-Host "cargo: $(cargo --version)"

Write-Host '=== BUILD ==='
cargo build --workspace --all-targets

Write-Host '=== CLIPPY ==='
cargo clippy --workspace --all-targets -- -D warnings

Write-Host '=== TESTS ==='
cargo test --workspace --all-targets

Write-Host '=== CLI SMOKE ==='
$tmp = Join-Path $env:TEMP ('one-verify-' + [guid]::NewGuid().ToString())
try {
    cargo run --quiet --package one-cli -- new $tmp
    Set-Content -Path (Join-Path $tmp 'src/main.one') -Value @('nama adalah "Budi"','tampilkan nama') -Encoding UTF8
    cargo run --quiet --package one-cli -- check (Join-Path $tmp 'src/main.one')
    cargo run --quiet --package one-cli -- run (Join-Path $tmp 'src/main.one')
    cargo run --quiet --package one-cli -- ast (Join-Path $tmp 'src/main.one') | Out-Null
    cargo run --quiet --package one-cli -- build (Join-Path $tmp 'src/main.one')
}
finally {
    if (Test-Path $tmp) { Remove-Item -Recurse -Force $tmp }
}

Write-Host 'OFFLINE VERIFICATION: PASS'
