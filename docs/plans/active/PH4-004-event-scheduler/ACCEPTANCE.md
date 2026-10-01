# PH4-004 Acceptance

- [x] EventId is a stable typed ID.
- [x] Events cannot be scheduled before current SimTick.
- [x] Same-tick events order by domain phase, then insertion sequence.
- [x] Cancelled events do not fire.
- [x] Rescheduling the same EventId invalidates its older queued generation.
- [x] AdvanceTo drains all valid due events through the requested tick.
- [x] Snapshot roundtrip preserves scheduler state.
- [x] Deterministic replay produces the same fired-event order and state hash.
- [x] Existing ledger/replay tests remain green.
- [x] Full GitHub CI passes.

## Evidence

GitHub Actions run 36876220349 passed:
- Rust;
- reference-model;
- DuckDB analytics;
- Postgres schema.
