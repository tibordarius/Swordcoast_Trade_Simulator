# PH4-006 Acceptance

- [x] Recipe/site/cohort/batch IDs are typed and stable.
- [x] Starting a production batch moves input from Holding inventory into WIP through EconomicTransaction.
- [x] Production completion occurs only from its scheduled event.
- [x] Completion consumes WIP and creates output through an explicit SourceOrSink account.
- [x] A production batch cannot complete twice.
- [x] Insufficient input rejects batch start without partial mutation.
- [x] Cohort consumption repeats on a deterministic interval.
- [x] Consumption serves available stock only and records unmet quantity separately.
- [x] Unmet demand is per-cycle data, not an ever-growing permanent price-pressure accumulator.
- [x] AdvanceTo is atomic if any due event fails.
- [x] Replay and snapshot roundtrip preserve production/population state.
- [x] Existing ScenarioPack/ledger/scheduler tests remain green.
- [x] Full GitHub CI passes.

## Evidence

GitHub Actions run 36888446229 passed:
- Rust, including hostile production/consumption integration tests;
- reference-model;
- DuckDB analytics;
- Postgres schema;
- v2 tiny ScenarioPack validation.
