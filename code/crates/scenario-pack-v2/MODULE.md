# scenario-pack-v2 Module Contract

## Purpose
Parse, validate, and compile external ScenarioPack v2 data into deterministic initialization commands for sim-kernel-v2.

## Owns
- external pack schema;
- validation;
- provenance/status classification at the pack boundary;
- deterministic initialization-command compilation.

## Does not own
- Forgotten Realms source interpretation;
- Google Drive/TorilGIS crawling;
- live simulation state;
- market/production/logistics behavior;
- frontend rendering.

## Public API
- load_json
- validate
- ValidatedScenarioPack
- compile_initialization_commands

## Dependencies
- serde / serde_json
- sim-kernel-v2 public API

## Forbidden dependencies
- v1 sim-core
- frontend
- Google Drive APIs
- database connections

## Invariants
- invalid external data cannot become executable commands;
- no direct WorldState mutation;
- command ordering is deterministic;
- opening balances use balanced EconomicTransaction postings.

## Focused tests
cargo test -p scenario-pack-v2
