# PH4-006 Plan

1. Add RecipeId, ProductionSiteId, ProductionBatchId, PopulationCohortId.
2. Extend scheduler records with typed EventPayload.
3. Refactor scheduler to pop one due event at a time.
4. Add production module: Recipe, ProductionSite, ProductionBatch.
5. Add population module: PopulationCohort, ConsumptionRecord.
6. Extend WorldState with production/population state and read-only queries.
7. Add reducer commands to register recipes/sites/cohorts and start batches.
8. Process domain event payloads during AdvanceTo on a staged clone.
9. Schedule recurring cohort consumption deterministically.
10. Add integration tests for production, shortage, repeated consumption, atomic advance, replay, and snapshots.
11. Run full repository CI.

Compatibility:
- existing generic scheduled events remain Noop events;
- PH4-003 ledger remains the only physical mutation path;
- PH4-005 ScenarioPack boundary remains unchanged;
- v1 remains untouched.
