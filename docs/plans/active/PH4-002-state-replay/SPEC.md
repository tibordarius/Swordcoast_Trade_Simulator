# PH4-002 Deterministic State and Replay

## Objective
Add the smallest authoritative v2 state/replay layer so the same initial state plus ordered command stream produces the same state hash.

## Scope
- WorldState foundation;
- world revision;
- command envelope;
- AdvanceTo command;
- reducer-only state mutation;
- deterministic replay;
- snapshot encode/decode;
- state hash tests.

## Non-goals
- event scheduler;
- ledgers;
- markets;
- shipments;
- ScenarioPack loader;
- persistence database.

## Invariants
- simulation time cannot move backwards;
- successful commands increment revision exactly once;
- command sequence must be strictly increasing during replay;
- snapshot roundtrip preserves stable state hash;
- public callers cannot directly mutate authoritative state fields.
