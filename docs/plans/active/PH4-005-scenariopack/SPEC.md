# PH4-005 Minimal ScenarioPack v2

## Objective
Create the first executable, versioned ScenarioPack boundary between TorilGIS-style reviewed data and sim-kernel-v2.

## Scope
- ScenarioPack manifest/schema version;
- immutable ScenarioRegistry;
- commodities, places, routes;
- account declarations and opening balances;
- explicit provenance/status classification;
- validation for IDs, references, units, routes, balances, and schema version;
- deterministic world initialization through reducer commands and balanced seed transactions;
- JSON encode/decode;
- tiny-world fixture and tests.

## Non-goals
- production recipes;
- population cohorts;
- markets/pricing;
- GIS geometry;
- live TorilGIS exporter;
- database persistence.

## Invariants
- runtime initialization never directly mutates WorldState;
- every opening balance is created through a balanced EconomicTransaction;
- source/campaign status is explicit;
- unknown/dangling IDs block loading;
- routes reference existing places;
- active commodities have valid freight dimensions;
- same ScenarioPack produces the same registry/world state hash.
