# Windows Native Verification — CI Provider Contract

The historical design proposed running the Windows native verification workflow from a temporary public verification repository. That arrangement is superseded for the canonical Project ONE CI path by the accepted CircleCI provider decision (ADR-0017).

## Canonical Project ONE workflow

- Canonical source remains the private repository `panglimanetizen01/one`.
- CircleCI configuration: `.circleci/config.yml`.
- Workflow: `windows-native-verification`.
- Trigger parameter: `run_windows_native_verification=true`.
- The job provisions MSYS2/MinGW-w64 and the pinned Android NDK r27c toolchain, runs the locked-equivalent workspace test/build checks without Cargo's `--locked` flag while no `Cargo.lock` is committed, builds the Windows GNU native target, validates the PE32+ AMD64 header, executes the artifact on a Windows executor, and stores the executable as a CircleCI artifact.

## Independent verification boundary

The separate public verification repository remains supplemental infrastructure for independent exact-SHA verification when required. It is not the canonical CI provider and must not replace or mislabel CircleCI job evidence.

## Evidence status

The CircleCI workflow configuration exists on the migration branch. Exact-SHA Windows job execution, PE artifact inspection, runtime execution, and artifact retrieval remain pending until CircleCI reports a successful run and the logs/artifact are inspected. Do not claim Windows native execution based only on configuration presence or a successful cross-build.
