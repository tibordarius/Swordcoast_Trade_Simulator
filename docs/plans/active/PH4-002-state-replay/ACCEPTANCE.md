# PH4-002 Acceptance

- [x] WorldState fields are not publicly mutable.
- [x] AdvanceTo cannot regress time.
- [x] Successful command increments revision once.
- [x] Replay rejects non-monotonic command sequence.
- [x] Same initial state + commands produces identical state hash.
- [x] Snapshot encode/decode preserves state hash.
- [x] Unsupported snapshot format is rejected.
- [x] cargo test -p sim-kernel-v2 passes through the Rust CI job.
- [x] full GitHub CI passes after rerunning one transient PostGIS service initialization failure.

## Evidence

GitHub Actions run 36868895161:
- Rust: success.
- reference-model: success.
- DuckDB analytics: success.
- Postgres schema: success on rerun.

The first Postgres attempt failed because the service container terminated during initialization before migration 0001 could connect. No branch code change was required.
