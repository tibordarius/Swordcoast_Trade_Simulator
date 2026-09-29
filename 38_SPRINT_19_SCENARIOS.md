# [[Sprint 19 — Scenario Branches]]

## Status

**Reference-complete.**

## Branching model

A scenario branch starts from a durable parent snapshot and then receives its own:

- branch ID;
- deterministic seed namespace;
- event log;
- simulation clock;
- derived state.

The canonical campaign state is never mutated.

## Validation

A thirty-day route-blockade counterfactual was run twice from the same snapshot and seed namespace. Both runs ended at the same state hash.

A baseline branch diverged from the blockade branch as expected.

The canonical parent hash was unchanged before and after both simulations.

## Monte Carlo

A 100-run blockade batch used independently derived deterministic seeds.

Reference final WDEX grain prices:

```text
minimum: 4,118 mcp
median:  4,192 mcp
maximum: 4,276 mcp
mean:    4,188.45 mcp
```

These results are scenario-lab outputs, not predictions of player behavior.
