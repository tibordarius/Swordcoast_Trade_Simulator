# PH4-005 Minimal ScenarioPack v2

## Objective
Create the first strict external-data boundary for sim-kernel-v2 and prove a tiny Toril corridor can be loaded, validated, and initialized through authoritative reducer/ledger commands.

## Scope
- standalone `scenario-pack-v2` crate;
- JSON loader;
- manifest/schema version;
- provenance/status classification;
- units;
- commodities;
- markets;
- routes;
- declared inventory/money accounts;
- opening inventory/money balances;
- deterministic validation;
- deterministic initialization command compilation;
- tiny 3-market / 5-good fixture;
- CLI validator and xtask entry point.

## Non-goals
- TorilGIS live export;
- production recipes;
- population cohorts;
- route pathfinding;
- map geometry;
- database persistence;
- source-text crawling.

## Invariants
- stable IDs are unique;
- all references resolve;
- active units have valid non-zero conversion ratios;
- routes reference valid distinct markets and positive distances;
- opening balances reference declared accounts/commodities;
- opening balances are non-negative;
- reserved system IDs cannot be supplied by the pack;
- initialization uses reducer commands and balanced EconomicTransaction postings;
- identical pack input compiles to identical command order and state hash.
