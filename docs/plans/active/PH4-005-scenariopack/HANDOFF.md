# PH4-005 Handoff

## Goal
Build the first minimal executable ScenarioPack boundary and deterministic world initializer.

## Status
REVIEW

## Branch
`phase4/v2-scenariopack`

## Completed
- ScenarioPack remains a standalone crate;
- accidental duplicate kernel-side pack implementation removed;
- typed ScenarioPackId and strengthened runtime registry contracts;
- explicit provenance/data status and place kind;
- commodity mass/volume and route travel ticks;
- campaign epoch and executable pack fields;
- strict validation for schema/IDs/references/units/dimensions/routes/opening balances;
- provenance mapped into immutable runtime registry;
- Waterdeep–Neverwinter–Luskan tiny fixture upgraded;
- deterministic initialization compilation;
- balanced seed source/external postings;
- safe `initialize()` entry point;
- JSON, metadata, invalid-pack, deterministic-load and seed-balancing tests;
- full repository CI verification.

## Tests last run
GitHub Actions run 36886057579: all jobs successful.

## Current failure / blocker
None.

## Next action
Review branch diff, open PR, and merge if scope remains limited to ScenarioPack/runtime-registry boundary.

## Do not redo
Do not move JSON/schema parsing into sim-kernel-v2. The standalone crate boundary is intentional.
