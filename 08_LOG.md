# Project Log

## 2026-09-29 — Project formalised

Created the [[Sword Coast Economic Simulator]] project workspace.

### Current concept

Build a live economic model of settlements and cities across the Sword Coast, including:

- resource origins;
- production;
- consumption;
- inventories;
- commodity prices;
- trading volume;
- physical trade routes;
- shipments;
- merchant organizations;
- exchange-style trading interfaces;
- eventual map overlay;
- campaign events that alter the economy.

### Working exchange concept

The first exchange is [[WDEX - Waterdeep Exchange]], inspired by live retail trading interfaces but backed by a physical economy.

### Architecture direction

Selected direction:

- Rust simulation core;
- React/TypeScript UI;
- PostgreSQL/PostGIS operational state;
- Timescale or equivalent time-series layer;
- DuckDB + Parquet analytics;
- MapLibre + deck.gl visualisation;
- PMTiles for static fantasy map data;
- deterministic replay and world branching.

### Key principle

Do not randomise prices directly. Random events change the world; the economy changes prices.

### Initial delivery plan

Two-week sprints beginning 29 September 2026. MVP target after the first 21 numbered sprints, with the engine, WDEX, six markets, map, analytics, DM controls and scenario branching included in the target scope.

## 2026-09-29 — Reference vertical slice reaches Sprint 6 validation

- implemented inventory reservation, shipment departure, ETA, derived progress and idempotent arrival;
- implemented market buy and sell slippage using fixed-point quantities;
- corrected an internal quantity-scale ambiguity before merchant integration;
- implemented merchant opportunity evaluation with capital, freight, route capacity, expected cargo loss and ROI threshold;
- static arbitrage test moves three 100,000 kg grain cargoes before the spread falls below the 5% ROI threshold;
- built an integrated 120-day ATHEX/WDEX grain benchmark;
- discovered that two concurrent 18-day shipments are insufficient to clear Waterdeep's structural deficit despite massive Athkatlan surplus;
- expanded logistics throughput from two to eight concurrent shipments and observed sharply lower unmet demand and average scarcity price;
- added a consolidated Python reference regression suite; all Sprint 1–6 reference tests pass.

## 2026-09-29 — Persistence, live protocol, WDEX and map foundation pulled forward

- added PostgreSQL 18/PostGIS 3.6 schema migrations for world, branches, provenance, settlements, commodities, markets, shipments, orders, event log and snapshots;
- added append-only protections for input events and trade executions;
- validated snapshot + event replay semantics with a standard-library persistence oracle;
- defined WebSocket/HTTP live-state contracts with monotonic sequence recovery and decimal-string encoding for 64-bit values;
- built a dependency-free WDEX market shell with exact BigInt formatting and order-preview slippage;
- implemented wallet, warehouse, limit-order reservation and warehouse-to-vessel physical cargo semantics;
- created a six-exchange grain network from CALEX to LUSEX and evaluate executable spreads after slippage, multi-hop freight, route risk and route capacity;
- recovered the existing campaign map configuration from `SwordCoastMap.md`: 6600×10200 px, center 3300/5100, scale 0.275 km/px;
- confirmed the actual referenced JPG is not in the uploaded campaign ZIP; settlement coordinates remain intentionally unguessed;
- built a local-only map calibration page that records Calimport, Athkatla, Baldur's Gate, Waterdeep, Neverwinter and Luskan directly from the user's existing map;
- consolidated all executable Sprint 0–12 checks into one regression suite.


## 2026-09-29 — Toril GIS adopted

- Reviewed Geospatial Grimoire Toril GIS and coordinate-system documentation.
- Adopted Toril GCS / FRIA as canonical storage geography.
- Added Myth Drannor display-longitude conversion.
- Pinned upstream repo commit `8f9bffae356ec32ec6f76ce788abf700d6e56c3b` and export batch `2026-07-30_1`.
- Recovered exact Toril GIS coordinates for Waterdeep, Baldur's Gate, Calimport, Neverwinter and Luskan.
- Confirmed current populated-place export does not name Athkatla; added a clearly provisional mapping-only coordinate with 30 km uncertainty from a secondary dataset transform.
- Added custom PostGIS SRID 910001 for Toril GCS.
- Implemented Toril Vincenty distance and client-side route interpolation.
- Built straight-geodesic route geometry seed and live logistics GeoJSON layer.
- Client benchmark derived 10,000 shipment positions in roughly 30 ms.
- Added dependency-free live logistics browser prototype.


## 2026-09-29 — Sprint 13b published pathway routing

- parsed the pinned Toril GIS `srf_civ_pathways_ln` SVG;
- confirmed 329 pathway features: 324 Primary Road and 5 Underwater Tunnel;
- found 42 pathway features intersecting the initial Sword Coast economic clip;
- added a reproducible pathway-to-routing-graph builder using actual published vertices;
- validated exact city attachment at Luskan, Neverwinter and Waterdeep;
- measured published overland geometry at ~233 km Luskan→Neverwinter and ~529 km Neverwinter→Waterdeep;
- kept maritime routing as a separate graph; straight sea connections remain placeholders only.


## 2026-09-29 — Sprint 14 market history

- added exact fixed-point OHLCV aggregation for 1H, 1D, 10D and 30D windows;
- added PostgreSQL `market_tick` and `market_candle` schema;
- preserved 64-bit money/quantity values through BigInt/string boundaries;
- aggregated 100,000 reference observations into hourly and daily candles in ~117 ms on the current runtime;
- added optional Timescale hypertable setup;
- added DuckDB SQL contracts for partitioned ZSTD Parquet archival and cross-market historical spread queries;
- DuckDB cannot be executed in the current sandbox because the package/binary is unavailable and external package installation is blocked.


## 2026-09-29 — Sprints 15–20 reference implementation

- formalized coarse immutable Parquet archive layout and DuckDB research-query contracts;
- added native DuckDB/Parquet CI gate with BIGINT round-trip above JavaScript safe-integer range;
- corrected GitHub Actions layout so the eventual repository root contains the whole project pack;
- added explicit production-recipe/site/inventory/job schema and an iron-chain shock propagation test;
- added data-driven independent/OTC/DTC merchant strategy profiles and benchmarked distinct opportunity choices;
- added DM world-event contract with direct-price-override rejection and causal price-evidence traces;
- added deterministic snapshot-descendant scenario branches and a 100-run Monte Carlo reference batch;
- added 2,000-operation randomized trading invariant test and chained-event recovery/tamper test;
- consolidated all locally executable checks through Sprint 20 into one reference suite;
- marked the project as an MVP candidate pending native Rust/PostGIS/DuckDB/end-to-end gates.


## 2026-09-29 — Sprint 21 release preparation

- added `tools/preflight.py` to report Rust, Docker, PostgreSQL client, Node and DuckDB availability;
- added strict and reference-only release gates;
- added project-root `Makefile` commands for reference tests, preflight, Rust tests and static prototype serving;
- reference release gate passes;
- current runtime correctly reports missing cargo/rustc/docker/psql/duckdb and therefore blocks native release rather than silently downgrading checks.
