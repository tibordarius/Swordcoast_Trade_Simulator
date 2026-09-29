# Architecture

## Architectural Position

Start with a **modular monolith plus one authoritative simulation writer per campaign world**. Avoid microservices until a measured bottleneck requires them.

The architecture should separate three concerns:

1. **Hot operational state** — current markets, inventories, shipments, orders and events.
2. **Simulation computation** — deterministic economic calculations in memory.
3. **Historical analytics** — large time-series queries, charts, comparisons and scenario analysis.

## Recommended Stack

| Layer | Technology | Role |
|---|---|---|
| Frontend | React + TypeScript | Exchange UI, dashboards, DM control room |
| Base map | MapLibre GL | Toril/Sword Coast map rendering |
| Dynamic map layers | deck.gl | trade flows, ships, heatmaps, animated routes |
| Charts | Lightweight Charts or ECharts | exchange-style OHLCV and market views |
| Browser analytics | DuckDB-Wasm | historical analysis without repeated API calls |
| Bulk data format | Apache Arrow | efficient analytical data transfer |
| API | Rust + Axum | HTTP/WebSocket command and query layer |
| Simulation core | Rust | deterministic high-performance engine |
| Parallelism | Rayon | data-parallel phases where deterministic reduction is possible |
| Operational database | PostgreSQL | durable current world state |
| Spatial database | PostGIS | settlement, region and route geometries |
| Routing | pgRouting initially | shortest/risk-adjusted route calculation |
| Time series | TimescaleDB | recent ticks, OHLCV, continuous aggregates |
| Historical analytics | DuckDB | Parquet and Postgres analytics |
| Historical archive | Parquet | compressed immutable history |
| Object storage | S3/R2-compatible | Parquet, snapshots, PMTiles |
| Static map package | PMTiles | map tiles without operating a tile server |
| Optional offline mode | PGlite + Rust/WASM | later self-contained browser campaign mode |
| Optional messaging | NATS JetStream | only if services become distributed |

## Data Flow

```text
React / Map / WDEX
        |
 HTTP + WebSocket
        |
 Rust API
        |
 authoritative simulation engine
        |
  +-----+----------------+
  |                      |
PostgreSQL/PostGIS    TimescaleDB
  |                      |
  +----------+-----------+
             |
        Parquet archive
             |
           DuckDB
             |
   historical/scenario analytics
```

## Key Performance Principles

### Multi-rate simulation

Different systems run at different frequencies. Market clearing may run every few in-world minutes, production hourly, merchant strategy every few hours, population every tenday, and harvest effects seasonally.

### Discrete-event logistics

A shipment stores departure time, route, ETA and state. The frontend derives its current position mathematically. Do not persist a new ship coordinate every visual frame.

### Single authoritative writer

One worker owns each campaign world's state transition order. Safe calculations may run in parallel, but changes are reduced and committed deterministically.

### In-memory simulation state

PostgreSQL is durable storage, not the inner simulation loop. Load compact arrays/structures into memory, calculate deltas, commit in batches.

### Structure-of-Arrays where useful

Large homogeneous numerical state should prefer cache-friendly arrays such as `market_price[]`, `inventory[]`, `route_risk[]`, and `shipment_eta[]` rather than deeply nested object graphs.

### Fixed-point economics

Represent currency and commodity quantity with integers or fixed-point units. Avoid floating-point drift in deterministic replay.

### Delta streaming

After an initial snapshot, stream only changes such as `market_delta`, `shipment_created`, `shipment_arrived`, `route_changed`, and `event_started`.

### Hot / warm / cold history

- Hot: Postgres/Timescale recent market state.
- Warm: compressed Timescale chunks and aggregates.
- Cold: Parquet in object storage.
- Analysis: DuckDB querying Parquet and, when needed, PostgreSQL.

## Technologies to Benchmark, Not Assume

- DuckDB native vs Polars for batch transforms.
- DuckDB-Wasm cost on lower-spec browsers.
- TimescaleDB vs vanilla Postgres partitions for the actual expected volume.
- pgRouting vs an in-memory Rust graph once route count grows.
- Arrow over WebSocket vs compressed MessagePack/JSON for live deltas.
- PMTiles performance for the chosen custom Toril map resolution.

The project should only add infrastructure when benchmark data justifies it.

## Things Not to Add Initially

- Kubernetes.
- Kafka.
- Elasticsearch.
- a graph database.
- ClickHouse.
- Redis unless an actual caching/distributed coordination need appears.
- microservices.

They remain options, not defaults.
