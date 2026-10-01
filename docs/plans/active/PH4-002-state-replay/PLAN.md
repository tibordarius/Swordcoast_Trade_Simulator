# PH4-002 Plan

1. Add WorldRevision and WorldState.
2. Add CommandEnvelope and initial AdvanceTo command.
3. Add WorldReducer with time-regression rejection.
4. Add deterministic replay helper with sequence validation.
5. Add versioned bincode snapshot envelope.
6. Add stable state hashing from canonical serialized state.
7. Add focused tests for replay and roundtrip.
8. Update xtask test-v2 coverage automatically through crate tests.

Compatibility:
- v1 untouched;
- no external data/schema changes.
