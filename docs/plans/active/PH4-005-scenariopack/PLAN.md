# PH4-005 Plan

1. Add ScenarioPackId, PlaceId, RouteEdgeId.
2. Add scenario module with schema structs.
3. Add immutable ScenarioRegistry.
4. Add validation report/error types.
5. Add deterministic loader that opens declared accounts through WorldReducer.
6. Create reserved seed source/external accounts and balanced seed transactions.
7. Add JSON roundtrip.
8. Add tiny three-market/five-good fixture.
9. Add validation and deterministic-load tests.
10. Run full repository gate.

Compatibility:
- v1 untouched;
- existing v2 snapshots/replay remain green;
- no direct WorldState mutation from loader.
