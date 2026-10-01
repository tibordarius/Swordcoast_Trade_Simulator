# PH4-006 Production and Cohort Consumption

## Objective
Create the first autonomous physical economy loop in sim-kernel-v2: scheduled Grain-to-Flour production and recurring cohort food consumption, with every physical change reconciled through the inventory ledger.

## Scope
- production/recipe/site/batch IDs and state;
- deterministic production recipes;
- production batch start with input reservation into WIP;
- scheduler payload for batch completion;
- completion converts input into output through explicit SourceOrSink accounts;
- population cohort state;
- recurring scheduled consumption cycles;
- partial consumption when stock is insufficient;
- explicit unmet-demand records;
- deterministic event dispatch during AdvanceTo;
- replay/snapshot coverage.

## Non-goals
- labor;
- wages;
- market prices;
- production investment;
- recipe substitution;
- multiple-input recipes;
- individual households;
- nutrition classes;
- TorilGIS export changes.

## Invariants
- production input is reserved before completion;
- a batch cannot complete twice;
- every production/consumption quantity change is ledger-posted;
- Holding accounts never go negative;
- shortage produces unmet demand rather than negative inventory;
- recurring consumption scheduling is deterministic;
- AdvanceTo is externally atomic: an event-processing error commits no partial state.
