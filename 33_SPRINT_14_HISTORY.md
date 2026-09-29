# Sprint 14 — Historical Markets

**Status:** Reference complete  
**Date:** 2026-09-29

## Goal

Give [[WDEX]] and the other exchanges a durable, exact price/volume history without forcing every chart or research query to replay raw simulation events.

## Hot history

`market_tick` stores recent authoritative observations:

- bid / ask / last in integer milli-copper;
- executed volume in fixed-point commodity units;
- closing inventory;
- imports and exports;
- authoritative simulation tick;
- market and commodity identity.

No authoritative financial quantity uses browser floating point.

## Candle aggregation

Implemented exact OHLCV aggregation for:

| UI interval | Simulation ticks |
|---|---:|
| 1H | 12 |
| 1D | 288 |
| 10D | 2,880 |
| 30D | 8,640 |

The `30D` window is intentionally called `30D`, not `1M`, until the Calendar of Harptos and festival-day handling are represented explicitly.

## Performance reference

100,000 raw observations:

- hourly candles: 8,334;
- daily candles: 348;
- reference aggregation: roughly 117 ms on the current runtime.

The browser implementation uses BigInt and decimal-string transport, so values above JavaScript's safe integer limit remain exact.

## Storage architecture

### Hot

PostgreSQL / optional Timescale:

- recent raw market ticks;
- current OHLCV/candle history;
- high-write operational access.

### Warm

Compressed/aggregated relational history.

### Cold

Parquet partitioned primarily by simulation-time bucket and market.

DuckDB queries Parquet directly for long-history research rather than copying the archive back into PostgreSQL.

## Optional Timescale

The core schema does **not** require TimescaleDB. An optional SQL setup converts `market_tick` into a hypertable when the deployment has the extension.

This avoids making local/offline or simpler PostgreSQL deployments depend on Timescale.

## DuckDB contract

Added SQL for:

- ZSTD Parquet archival;
- Hive-style partitioning;
- historical WDEX/BGEX commodity spread research;
- direct analytical scans of archived history.

DuckDB itself is not executable in the current sandbox because neither the binary nor Python package is installed and external package installation is blocked. The SQL contract is therefore present but not falsely marked runtime-tested here.

## Exit gate

- OHLCV aggregation deterministic and exact;
- 64-bit values remain lossless;
- hot-history schema exists;
- Parquet/DuckDB archival contract exists;
- history tests included in global regression suite.
