# Native Roadmap

## Completed / validated

- deterministic Rust kernel
- fixed-point inventories/prices/slippage
- shipment primitives
- merchant opportunity calculation
- native Rust CI
- PostgreSQL/PostGIS migration CI
- DuckDB/Parquet native CI
- seed validation
- Axum API
- WebSocket world-status stream
- six live WDEX commodity states
- static WDEX browser shell
- Toril custom CRS schema

## Next: persistence

- connect API command handling to PostgreSQL
- persist input events before mutation
- periodic deterministic snapshots
- restore from snapshot + event log
- branch/scenario persistence

## Next: native trade network

- move market definitions out of API hard-coding and into validated seed loading
- instantiate all six MVP exchanges in Rust
- native route graph
- physical shipment scheduler
- merchant arbitrage loop
- market inventory transfers on departure/arrival
- route risk/loss events

## Next: exchange operations

- real market and limit order endpoints
- player/trader accounts
- warehouse positions
- physical cargo transfer
- transaction/event audit trail
- market depth generated from simulated liquidity

## Next: geography

- import pinned Toril GIS settlements/pathways
- create navigable overland graph
- design sea-lane graph
- live shipment GeoJSON
- MapLibre/deck.gl client
- commodity flow overlays

## Next: history

- write raw market ticks
- OHLCV aggregation
- Timescale optional acceleration
- Parquet archival
- DuckDB historical API/query worker

## Next: campaign control

- DM event injection
- causal `Why did this move?`
- scenario branches
- Monte Carlo scenario runner
- AI adapter only outside the deterministic boundary
