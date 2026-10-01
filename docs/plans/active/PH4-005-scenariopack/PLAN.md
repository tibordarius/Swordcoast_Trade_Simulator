# PH4-005 Plan

1. Keep `scenario-pack-v2` as the standalone external-data boundary.
2. Strengthen sim-kernel-v2 runtime registry types:
   - ScenarioPackId / PlaceId / RouteEdgeId / UnitId;
   - provenance/data status;
   - commodity mass + volume;
   - place kind;
   - route travel ticks.
3. Strengthen ScenarioPack v2 schema:
   - typed pack ID;
   - campaign epoch;
   - executable commodity dimensions;
   - place kind;
   - route travel time.
4. Validate schema version, IDs, references, units, physical dimensions, routes, opening balances, and reserved IDs.
5. Compile immutable ScenarioRegistry from validated pack data.
6. Compile deterministic initialization commands using balanced opening source/external accounts.
7. Expose one safe `initialize()` API that creates WorldState only through replay/WorldReducer.
8. Maintain a three-market/five-good Sword Coast fixture.
9. Test JSON roundtrip, metadata preservation, invalid packs, deterministic compilation, seed balancing, and one-step initialization.
10. Run full repository gate.

Compatibility:
- v1 untouched;
- existing v2 snapshots/replay remain green;
- sim-kernel-v2 does not own JSON or ScenarioPack parsing;
- scenario-pack-v2 does not directly mutate WorldState.
