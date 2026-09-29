# Architecture

## Goal

The Sword Coast Trade Simulator is a deterministic economic and logistics simulation. Prices emerge from production, consumption, inventories, market depth, route capacity, freight cost, risk, and merchant behaviour.

## Runtime boundaries

### `sim-core`

Rust library containing deterministic world mechanics.

Current responsibilities:
- five-minute authoritative clock
- named deterministic RNG streams
- stable event ordering
- stable state hashing
- inventory conservation
- fixed-point market pricing and slippage
- shipment lifecycle primitives
- merchant opportunity evaluation
- deterministic market-commodity state

Rules:
- no floating-point economic state
- no wall-clock time inside simulation logic
- no unseeded randomness
- one authoritative writer per world/branch
- same seed + simulation version + ordered input events => same state

### `wdex-api`

Axum service around one authoritative in-memory world.

Current endpoints:
- `GET /health`
- `GET /v1/worlds/{world_id}/status`
- `POST /v1/worlds/{world_id}/advance`
- `GET /v1/markets/{market_id}/snapshot`
- `GET /v1/live/{world_id}` (WebSocket)

All 64-bit quantities, ticks, prices, and hashes that cross JSON are represented as decimal/hex strings where precision matters.

### PostgreSQL + PostGIS

Durable operational state:
- worlds and branches
- settlements and markets
- inventories
- routes
- shipments
- orders/executions
- append-only command/event log
- snapshots
- warehouses/vessels
- market history
- production chains

PostGIS SRID `910001` represents Toril GCS using the FRIA prime meridian.

### DuckDB + Parquet

Long-horizon analytical layer:
- archived market history
- spread analysis
- volatility/volume research
- scenario comparisons
- large historical queries

PostgreSQL is not used as the primary research warehouse.

### Browser

`code/web/wdex/` is the current WDEX shell.

Current behaviour:
- exact BigInt formatting and order-preview slippage
- real engine tick/hash via HTTP/WebSocket
- six WDEX commodity rows overlaid from live Rust market state
- offline/reference fallback when API is unavailable

The browser must never use JavaScript Number for authoritative 64-bit economic quantities.

## Geography

Canonical coordinates come from Toril GIS, pinned by upstream commit. Campaign raster maps are display overlays, not the world coordinate system.

Planned map stack:
- MapLibre for base geography
- deck.gl for live trade flows, shipments, risk, and heat maps
- PostGIS for authoritative spatial state
- PMTiles for static layers

## Persistence model

The long-term recovery rule is:

`latest snapshot + ordered append-only input events = current state`

Current Rust API remains in-memory until the persistence adapter is connected. Database migrations are already tested natively in CI.

## Data flow

```
DM / Player
    |
    v
Axum API  ----> append-only command log (planned adapter)
    |
    v
single deterministic Rust writer
    |
    +--> PostgreSQL/PostGIS current state
    +--> live WebSocket deltas
    +--> market history
              |
              v
        Parquet archive
              |
              v
            DuckDB

Browser <---- HTTP/WebSocket ---- API
```
