# PH4-005 Acceptance

- [x] scenario-pack-v2 is a standalone workspace crate.
- [x] JSON pack loader parses the tiny fixture.
- [x] unsupported schema versions are rejected.
- [x] duplicate IDs are rejected.
- [x] dangling unit/market/account/commodity references are rejected.
- [x] zero/invalid unit conversion is rejected.
- [x] invalid routes are rejected.
- [x] reserved initialization IDs are rejected.
- [x] opening balances compile into balanced reducer transactions.
- [x] replaying compiled initialization commands is deterministic.
- [x] tiny fixture creates 3 markets and 5 commodities in the immutable ScenarioRegistry.
- [x] scenario-pack-validate CLI works.
- [x] cargo xtask pack-validate works in CI.
- [x] full GitHub CI passes.

## Evidence

GitHub Actions run 36878536869 passed:
- Rust;
- reference-model;
- DuckDB analytics;
- Postgres schema.

The Rust job also executed:
cargo xtask pack-validate crates/scenario-pack-v2/tests/fixtures/tiny_sword_coast.json
successfully.
