# Testing & Validation Strategy

## 1. Determinism Tests

Run identical seed/input scenarios multiple times and compare authoritative state hashes at checkpoints.

Failure means the engine is not ready for replay or branching.

## 2. Conservation Tests

Property tests for goods and money:

`opening_stock + production + imports - consumption - exports - losses = closing_stock`

within explicit transformation rules.

## 3. Economic Direction Tests

Examples:

- reducing supply while demand is unchanged should not lower equilibrium price without another documented cause;
- increasing route cost should reduce attractiveness of that route;
- a profitable persistent spread should attract merchant capacity;
- shipment arrivals should relieve shortages, all else equal;
- finite capacity should prevent instant arbitrage convergence.

## 4. Shock Scenarios

Maintain fixed benchmark scenarios:

- Amnian harvest failure;
- Luskan blockade;
- Mirabar iron collapse;
- Waterdeep festival demand surge;
- Pirate-risk spike;
- major convoy arrival;
- whale-oil supply disruption.

Expected behaviour should be described qualitatively before running the test.

## 5. Performance Benchmarks

Benchmark tiers:

### Small
- 6 markets;
- 25 commodities;
- 30 merchants;
- 100 active shipments.

### Sword Coast
- 100 settlements;
- 60 commodities;
- 300 merchants;
- 10,000 active shipments.

### Toril target
- 500+ settlements;
- 100+ commodities;
- 1,000 merchant actors;
- 100,000+ active shipments/commitments.

Measurements:

- ticks/second;
- memory;
- snapshot size/time;
- database write rate;
- replay speed;
- route-evaluation cost;
- WebSocket payload volume;
- map FPS at shipment counts.

## 6. Database Benchmarks

Compare:

- vanilla Postgres partitioning;
- Timescale hypertables;
- compressed/columnar retention where available;
- Parquet export size;
- DuckDB scan time.

Do not adopt infrastructure because it is fashionable. Use the smallest system that meets measured needs.

## 7. Browser Analytics Benchmark

DuckDB-Wasm tests:

- 100 MB Parquet;
- 500 MB Parquet;
- 1 GB Parquet where practical;
- Chrome/Firefox;
- representative laptop hardware;
- Worker memory pressure;
- query latency.

## 8. Map Benchmark

Test:

- 1,000 animated shipments;
- 10,000;
- 50,000;
- commodity filtering;
- flow lines;
- heatmaps;
- zoom transitions.

The frontend should interpolate motion; server update frequency should not determine visual smoothness.

## 9. Lore/Data Validation

Every seed dataset import should flag:

- missing provenance;
- impossible commodity units;
- duplicate settlement IDs;
- routes with invalid endpoints;
- production without a plausible location;
- consumption references to unknown commodities;
- conflicting canonical records.

Conflicts should be recorded, not silently reconciled.

## 10. Definition of Done for Engine Features

A simulation feature is not done until it has:

- deterministic test;
- invariant/property test where applicable;
- benchmark impact understood;
- documentation;
- replay compatibility;
- clear state/provenance representation.
