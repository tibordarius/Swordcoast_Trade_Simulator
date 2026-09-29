# Sprint 7 — Persistence

## Status

**Schema and replay semantics implemented at reference level. Native PostgreSQL execution is wired into CI but cannot be run in the current ChatGPT runtime.**

## Delivered

### PostgreSQL/PostGIS

- `code/migrations/0001_core.sql`
- `code/migrations/0002_append_only.sql`
- `code/migrations/9000_ci_assertions.sql`
- `code/compose.yaml`

The schema includes the first durable forms of:

- world and branch;
- provenance;
- region and settlement geometry;
- commodities;
- markets;
- branch-specific market inventory and current price state;
- routes with spatial geometry;
- traders/accounts;
- shipments;
- orders and immutable trade executions;
- append-only external input events;
- snapshots.

Economic quantities stay integer/fixed-point in persistence: `*_milli`, `*_mcp`, and `*_bps`. PostgreSQL floating-point types are deliberately excluded from authoritative economic state.

### Append-only event log

The PostgreSQL migration installs triggers that reject `UPDATE` and `DELETE` on `input_event_log` and `trade_execution`.

### Snapshot + replay reference

`reference/sprint7_persistence.py` uses SQLite only as an executable semantic oracle. It verifies:

1. external commands are appended with monotonic sequence numbers;
2. a canonical snapshot is checksummed;
3. later events can replay from the snapshot;
4. restored final state exactly equals live final state;
5. attempts to mutate the event log fail.

### Migration lint

`tools/lint_migrations.py` checks required tables, spatial enablement, append-only triggers and fixed-point conventions without pretending to be a PostgreSQL parser.

## Reference result

`SPRINT7_PERSISTENCE: PASS`

Final reference state:

- tick: `288`
- grain: `2,175,000,000 milli-units`
- cash: `955,000,000 mcp`
- route risk: `900 bps`
- state SHA-256: `e82a087d45c791526365988cf66d748c142e334bb0e0aae5407638d53473914a`

## Exit gate still pending

Run all migrations against actual PostgreSQL/PostGIS and make the native Rust persistence adapter pass save → restart → restore → continue without deterministic divergence.
