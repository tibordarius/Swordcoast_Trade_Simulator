# Development

## Native gates

GitHub Actions currently validates:

1. Rust formatting/bootstrap, Clippy with warnings denied, and all workspace tests.
2. Seed validation.
3. DuckDB/Parquet BIGINT round-trip and analytical checks.
4. PostgreSQL/PostGIS migrations and schema assertions.
5. WDEX fixed-point browser tests.

## Run Rust tests

```bash
cd code
cargo test --workspace --all-targets
```

## Run the API

```bash
cd code
cargo run -p wdex-api
```

Defaults:
- bind: `127.0.0.1:3000`
- world: `WORLD-SWORD-COAST-V0`
- seed: `12345`

Environment overrides:
- `WDEX_BIND`
- `WDEX_WORLD_ID`
- `WDEX_WORLD_SEED`

## Run WDEX

Serve the static browser files:

```bash
python -m http.server 8080 --directory code/web
```

Then open:

`http://127.0.0.1:8080/wdex/`

To point at another API:

`http://127.0.0.1:8080/wdex/?api=http://host:3000`

## Useful endpoints

```text
GET  /health
GET  /v1/worlds/WORLD-SWORD-COAST-V0/status
POST /v1/worlds/WORLD-SWORD-COAST-V0/advance
GET  /v1/markets/MKT-WD/snapshot
WS   /v1/live/WORLD-SWORD-COAST-V0
```

Advance payload:

```json
{"ticks":"288"}
```

One day is 288 five-minute ticks.

## Database

CI applies all `code/migrations/0*.sql` files against a real PostGIS container and then runs `9000_ci_assertions.sql`.

## Current limitation

The API currently owns world state in memory. PostgreSQL schemas are ready, but snapshot/event-log persistence is the next integration milestone.
