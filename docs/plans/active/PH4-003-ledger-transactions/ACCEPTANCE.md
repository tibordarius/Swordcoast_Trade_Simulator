# PH4-003 Acceptance

- [x] Accounts can only be opened through reducer commands.
- [x] Inventory and money postings are balanced.
- [x] Holding accounts cannot become negative.
- [x] Explicit source/external accounts may carry balancing negative totals.
- [x] Duplicate transaction IDs are rejected.
- [x] Invalid mixed transaction changes neither goods nor money.
- [x] Successful transaction writes authoritative ledger entries.
- [x] Replay with transactions is deterministic.
- [x] Snapshot roundtrip preserves transaction/ledger state.
- [x] cargo test -p sim-kernel-v2 passes through the Rust CI job.
- [x] full GitHub CI passes.

## Evidence

GitHub Actions run 36870329971 passed:
- Rust;
- reference-model;
- DuckDB analytics;
- Postgres schema.
