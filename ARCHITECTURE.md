# Toril Economy Architecture

## Current direction

The repository contains two generations:

1. **v1 reference implementation** in the existing workspace.
2. **v2 authoritative kernel** under `code/crates/sim-kernel-v2`.

v1 remains runnable and useful for regression, behavioral comparison, and migration fixtures.

## Core v2 boundary

```text
TorilGIS / Scenario authoring
        |
        v
Versioned ScenarioPack
        |
        v
sim-kernel-v2
  - typed values
  - scheduler
  - transactions
  - ledgers
  - reducer
  - markets
  - information
  - logistics
        |
        +--> snapshots/replay
        +--> domain events
        +--> analytics
        |
        v
query/API/UI
```

## Authoritative state rules

- `ScenarioRegistry` is immutable for a simulation lineage.
- `WorldState` contains mutable runtime state.
- Commands propose changes.
- Transactions validate conserved changes.
- One reducer commits them in deterministic order.
- Domain events describe committed results.
- Query/UI layers are read-only with respect to authoritative state.

## Time and determinism

Simulation time uses integral ticks.
Randomness will be keyed/domain-separated.
The same ScenarioPack, seed, command stream, and engine version must reproduce the same state hash.

## Economic arithmetic

Money, quantity, and authoritative unit prices use integral fixed-point representations.
Floating point is allowed only in non-authoritative presentation/analysis unless an ADR explicitly changes this rule.

## Integration

TorilGIS owns source identity, provenance, geography, registry review, and scenario authoring.
v2 owns dynamic causal state.
The bridge is a versioned ScenarioPack, not direct source crawling.

## Modularity

Each domain module should own:
- explicit state;
- explicit public API;
- invariants;
- focused fixtures/tests;
- no hidden global mutation.

See `docs/templates/MODULE.md`.
