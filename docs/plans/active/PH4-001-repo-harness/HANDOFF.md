# PH4-001 Handoff

## Goal
Create the Phase 4 construction harness and first v2 Rust foundation.

## Status
REVIEW

## Branch
`phase4/v2-foundation`

## Completed
- root AGENTS.md;
- current ARCHITECTURE.md;
- feature/handoff/module templates;
- accepted ADRs;
- Drive research index;
- `sim-kernel-v2` workspace crate;
- typed SimTick, MoneyCp, Quantity, UnitPrice and stable IDs;
- deterministic SplitMix64/domain stream seed helper;
- deterministic hash utility;
- `cargo xtask` command harness;
- workspace registration.

## Tests last run
GitHub Actions run 36867765039:
- Rust: success;
- reference-model: success;
- DuckDB analytics: success;
- Postgres schema: success.

## Current failure / blocker
None.

## Decisions made
ADR-0001, ADR-0002 and ADR-0003 are accepted.

## Important files
- AGENTS.md
- ARCHITECTURE.md
- code/crates/sim-kernel-v2/
- code/tools/xtask/
- docs/decisions/

## Next action
Review the branch diff and merge PH4-001 if the changes remain limited to the harness/v2 foundation.

## Remaining acceptance criteria
None.

## Do not redo
Do not reopen v1-versus-v2 architecture without material new evidence.
