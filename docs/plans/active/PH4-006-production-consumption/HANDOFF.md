# PH4-006 Handoff

## Goal
Implement the first scheduled physical production and population-consumption loop.

## Status
REVIEW

## Branch
`phase4/v2-production-consumption-impl`

## Supersedes
The earlier `phase4/v2-production-consumption` branch was created before PH4-005 merged. Its reviewed ID and scheduler-payload work was carried forward onto this clean branch; the stale branch should not be resumed.

## Completed
- typed recipe/site/batch/cohort IDs;
- typed scheduler payloads and Population event domain;
- production recipes, sites and batch state;
- population cohorts and per-cycle consumption records;
- input reservation into WIP via EconomicTransaction;
- scheduled production completion;
- explicit SourceOrSink conversion accounting;
- recurring cohort consumption;
- partial serving and per-cycle unmet demand;
- staged-clone atomic AdvanceTo event processing;
- snapshot format v4;
- deterministic replay/snapshot tests;
- hostile WIP-drain rollback test;
- full repository CI verification.

## Tests last run
GitHub Actions run 36888446229: all jobs successful.

## Current failure / blocker
None.

## Next action
Review branch diff, open PR, and merge if scope remains limited to production/consumption foundation.

## Do not redo
Do not add price formation or market demand into this increment. Consumption is physical serving/unmet demand only.
