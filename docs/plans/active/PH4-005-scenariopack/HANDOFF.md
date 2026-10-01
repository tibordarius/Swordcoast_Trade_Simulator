# PH4-005 Handoff

## Goal
Build the first minimal executable ScenarioPack boundary and deterministic world initializer.

## Status
VERIFY

## Branch
`phase4/v2-scenariopack`

## Completed
- confirmed ScenarioPack remains a standalone crate;
- removed accidental duplicate ScenarioPack implementation from sim-kernel-v2;
- added ScenarioPackId and strengthened runtime registry contracts;
- added explicit DataStatus and PlaceKind;
- added commodity mass/volume and route travel ticks;
- strengthened standalone pack schema with campaign epoch and executable fields;
- pack validator now checks physical dimensions and travel time;
- registry compiler maps provenance into runtime metadata;
- tiny Waterdeep–Neverwinter–Luskan fixture upgraded;
- JSON/determinism/invalid-pack/seed-balancing tests added;
- safe one-step `initialize()` API added.

## Current verification
Latest full GitHub Actions run is in progress.

## Important files
- code/crates/scenario-pack-v2/
- code/crates/sim-kernel-v2/src/registry.rs
- code/crates/sim-kernel-v2/src/ids.rs
- code/crates/scenario-pack-v2/tests/fixtures/tiny_sword_coast.json

## Next action
If final CI is green, mark acceptance complete, open PH4-005 PR, and merge.

## Do not redo
Do not move JSON/schema parsing into sim-kernel-v2. The standalone crate boundary is intentional and now proven by implementation.
