# PH4-005 Handoff

## Goal
Create the first executable ScenarioPack v2 boundary and tiny-world fixture.

## Status
REVIEW

## Branch
phase4/v2-scenario-pack

## Completed
- standalone scenario-pack-v2 crate;
- JSON schema/model;
- provenance status enum;
- strict deterministic validation;
- Unit/Commodity/Place/Market/Route specs;
- declared inventory and money accounts;
- opening inventory and money balances;
- reserved initialization accounts;
- deterministic initialization command compilation;
- immutable ScenarioRegistry in sim-kernel-v2;
- compile_registry bridge;
- 3-market / 5-commodity tiny Sword Coast fixture;
- scenario-pack-validate CLI;
- cargo xtask pack-validate;
- CI execution of the xtask validation command;
- failure tests for schema, duplicates, dangling refs, invalid units/routes, reserved IDs, and negative opening balances.

## Tests last run
GitHub Actions run 36878536869: all jobs successful.

## Current failure / blocker
None.

## Important files
- code/crates/scenario-pack-v2/
- code/crates/sim-kernel-v2/src/registry.rs
- code/crates/scenario-pack-v2/tests/fixtures/tiny_sword_coast.json
- code/tools/xtask/src/main.rs
- .github/workflows/ci.yml

## Next action
Review branch diff, open PR, and merge if scope remains limited to the ScenarioPack boundary and tiny-world initialization.

## Do not redo
Do not connect TorilGIS or Google Drive directly to sim-kernel-v2. Future live export work must produce this pack boundary.
