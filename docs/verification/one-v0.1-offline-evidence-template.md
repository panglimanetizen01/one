# One v0.1 Offline Verification Evidence

Copy this template once a clean Linux or Windows verification environment has executed the canonical repository.

## Identity

- Repository: `panglimanetizen01/one`
- Branch/ref:
- Commit SHA:
- Date/time UTC:
- Platform: `Linux` / `Windows`
- OS version:
- Rust version:
- Cargo version:

## Command

```text
bash tools/verify_offline.sh
```

or on Windows:

```text
./tools/verify_offline.ps1
```

## Result

- [ ] build passed
- [ ] clippy passed
- [ ] workspace tests passed
- [ ] CLI smoke passed
- [ ] conformance/golden tests passed
- [ ] no source mutation occurred

Overall result: `PASS` / `FAIL`

## Failure detail

If any check fails, record the exact command and first actionable error here. Do not label the repository verified until the failure is resolved and the complete verifier is rerun.

## Hosted CI distinction

This record is **independent execution evidence**. It must not be described as a GitHub-hosted Actions result. Hosted CI remains a separate evidence channel.
