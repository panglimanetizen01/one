$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
$env:CARGO_NET_OFFLINE = 'true'

function Invoke-CheckedNative {
    param(
        [Parameter(Mandatory)][string]$FilePath,
        [Parameter(Mandatory)][string[]]$Arguments
    )
    & $FilePath @Arguments
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        $display = "$FilePath $($Arguments -join ' ')"
        throw "Native command failed with exit code ${exitCode}: ${display}"
    }
}

Write-Host '=== NATIVE EXIT-CODE GUARD SELF-TEST ==='
try {
    Invoke-CheckedNative -FilePath 'cmd.exe' -Arguments @('/c', 'exit', '23')
    throw 'Native command exit-code self-test did not detect the expected failure.'
}
catch {
    if ($_.Exception.Message -notlike '*exit code 23*') { throw }
}
Write-Host 'Native command exit-code self-test: PASS'

Write-Host '=== ONE OFFLINE VERIFICATION ==='
try { Write-Host "commit: $(git rev-parse HEAD)" } catch { Write-Host 'commit: unknown' }
Invoke-CheckedNative -FilePath 'rustc' -Arguments @('--version')
Invoke-CheckedNative -FilePath 'cargo' -Arguments @('--version')

Write-Host '=== CONTRACT ARTIFACTS ==='
Invoke-CheckedNative -FilePath 'python' -Arguments @('tools/check_contract.py')

Write-Host '=== BUILD ==='
Invoke-CheckedNative -FilePath 'cargo' -Arguments @('build', '--locked', '--workspace', '--all-targets')

Write-Host '=== CLIPPY ==='
Invoke-CheckedNative -FilePath 'cargo' -Arguments @('clippy', '--locked', '--workspace', '--all-targets', '--', '-D', 'warnings')

Write-Host '=== TESTS ==='
$testArguments = @('test', '--locked', '--workspace', '--all-targets')
if ($env:OS -eq 'Windows_NT') {
    # The Android native target is exercised on the Ubuntu/NDK and ARM64 jobs.
    # This Windows host has no Android NDK sysroot, so do not run that one test here.
    $testArguments += @('--', '--skip', 'android_target_explicit_mempertahankan_build_native')
}
Invoke-CheckedNative -FilePath 'cargo' -Arguments $testArguments

Write-Host '=== CLI SMOKE ==='
$tmp = Join-Path $env:TEMP ('one-verify-' + [guid]::NewGuid().ToString())
try {
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'buat', $tmp)
    Set-Content -Path (Join-Path $tmp 'src/main.one') -Value @('nama adalah "Budi"','tampilkan nama') -Encoding UTF8
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'periksa', (Join-Path $tmp 'src/main.one'))
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'jalankan', (Join-Path $tmp 'src/main.one'))
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'lihat', 'struktur', (Join-Path $tmp 'src/main.one')) | Out-Null
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'bangun', (Join-Path $tmp 'src/main.one'))
    Copy-Item (Join-Path $tmp 'src/target/main.oneir') (Join-Path $tmp 'first.oneir')
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'bangun', (Join-Path $tmp 'src/main.one'))
    if ((Get-FileHash (Join-Path $tmp 'first.oneir')).Hash -ne (Get-FileHash (Join-Path $tmp 'src/target/main.oneir')).Hash) { throw 'oneir artifact is not deterministic' }
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'test', $tmp)
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'package', $tmp)
    Copy-Item (Join-Path $tmp 'target/proyek-one-0.1.0.onepkg') (Join-Path $tmp 'first.onepkg')
    Invoke-CheckedNative -FilePath 'cargo' -Arguments @('run', '--locked', '--quiet', '--package', 'one-cli', '--bin', 'one', '--', 'package', $tmp)
    if ((Get-FileHash (Join-Path $tmp 'first.onepkg')).Hash -ne (Get-FileHash (Join-Path $tmp 'target/proyek-one-0.1.0.onepkg')).Hash) { throw 'onepkg artifact is not deterministic' }
}
finally {
    if (Test-Path $tmp) { Remove-Item -Recurse -Force $tmp }
}

Write-Host 'OFFLINE VERIFICATION: PASS'