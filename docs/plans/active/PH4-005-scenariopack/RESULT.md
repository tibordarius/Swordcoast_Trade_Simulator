# PH4-005 Result

## Outcome
The first executable ScenarioPack v2 boundary is implemented as a standalone crate.

## Architecture
- `scenario-pack-v2` owns external JSON/schema/provenance/validation.
- `sim-kernel-v2` owns executable registry types and runtime state.
- initialization is compiled into deterministic reducer commands and replayed through WorldReducer.
- no direct WorldState seeding exists.

## Executable data now includes
- typed pack identity and campaign epoch;
- units and explicit conversions;
- commodities with mass and volume;
- place kind;
- markets;
- routes with distance and travel ticks;
- provenance/data status;
- inventory/money accounts;
- opening balances.

## Verification
GitHub Actions run 36886057579 passed all repository jobs.

## Follow-up
PH4-006 should add minimal production and consumption using the existing ledger and scheduler. Production/consumption must produce EconomicTransaction postings rather than mutate account balances.
