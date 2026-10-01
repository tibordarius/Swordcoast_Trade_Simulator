# scenario-pack-v2 Module Contract

## Purpose
Own the external ScenarioPack v2 schema, validate it, compile immutable runtime registry data, and initialize sim-kernel-v2 only through deterministic reducer commands.

## Owns
- external pack JSON schema;
- schema versioning;
- provenance/status classification at the pack boundary;
- executable unit/commodity/place/market/route records;
- opening account/balance declarations;
- validation;
- deterministic initialization-command compilation;
- the safe `initialize()` entry point.

## Does not own
- Forgotten Realms source interpretation;
- Google Drive/TorilGIS crawling;
- mutable simulation behavior after initialization;
- markets/production/logistics logic;
- frontend rendering;
- database connections.

## Public API
- `load_json`
- `validate_pack`
- `ValidatedScenarioPack`
- `compile_registry`
- `compile_initialization_commands`
- `initialize`
- `InitializedScenario`

## Dependencies
- serde / serde_json
- sim-kernel-v2 public API

## Forbidden dependencies
- v1 sim-core
- frontend
- Google Drive APIs
- database connections

## Invariants
- invalid external data cannot become executable runtime state;
- no direct WorldState mutation;
- initialization uses WorldReducer/replay only;
- command ordering is deterministic;
- opening balances use balanced EconomicTransaction postings;
- source/campaign/derived/generated status remains explicit;
- active commodities have executable physical dimensions;
- routes have explicit positive travel time and distance.

## Focused tests
`cargo test -p scenario-pack-v2`
