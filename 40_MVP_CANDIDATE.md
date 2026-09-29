# [[WDEX MVP Candidate]]

## Current state

The reference implementation is green through [[Sprint 20]]. This is an **MVP candidate**, not a production release.

## Functionally represented

- deterministic simulation clock and RNG streams;
- production and consumption;
- inventories, reserves and shortages;
- causal pricing, bid/ask depth and slippage;
- physical logistics and shipment ETA;
- autonomous merchant arbitrage;
- six connected exchanges;
- player wallet, warehouses and physical vessel cargo;
- canonical Toril GIS coordinate model;
- published overland pathway graph where available;
- live shipment interpolation without position spam;
- exact OHLCV history;
- DuckDB/Parquet analytical contract;
- multi-stage production chains;
- distinct [[OTC]] / [[DTC/DWTC (Deepwater Trading Company)]] strategies;
- DM event injection and causal price explanations;
- deterministic scenario branches and Monte Carlo batches;
- recovery, tamper and randomized ledger tests.

## Release blockers

The following must be green before calling this a runnable MVP release:

1. native Rust workspace compiles, formats, passes Clippy and passes all Rust tests;
2. PostgreSQL/PostGIS migrations run against a real server;
3. DuckDB writes and reads the Parquet archive successfully in CI;
4. a repository exists so GitHub Actions can execute the prepared gates;
5. frontend dependencies are bundled into a runnable app rather than only dependency-free prototypes/contracts;
6. production configuration and seed import are wired into the native service;
7. at least one full end-to-end run is performed through API → simulator → Postgres → WebSocket → WDEX/map;
8. provisional [[Athkatla]] mapping is either replaced by a stronger Toril GIS source or retained with visible provenance/uncertainty.

## Release definition

The first runnable MVP does **not** need all of Toril. It needs the six-market Sword Coast vertical slice to run end-to-end, survive restart/replay and visibly respond to a DM-injected shock without manual price editing.
