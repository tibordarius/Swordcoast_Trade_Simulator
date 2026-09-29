# [[Sprint 15 — DuckDB Analytics]]

## Status

**Reference-complete. Native DuckDB/Parquet gate prepared for CI but not executable in the current sandbox.**

## Delivered

- coarse Hive-style Parquet archive layout for [[WDEX]] market history;
- immutable archive manifest contract with row counts and SHA-256 hashes;
- canonical DuckDB queries for spreads, volumes and integer-friendly volatility;
- route ROI terminology fixed as **arrival mark-to-market ROI** until cargo-lot disposal attribution exists;
- safe browser analytics query contract using parameter objects rather than arbitrary SQL;
- native CI gate that writes/reads Parquet and checks 64-bit integer round-trips beyond JavaScript's safe integer range;
- repository-root CI layout corrected so Rust, reference models, migrations and analytics gates can coexist.

## Archive decision

Partition by coarse dimensions:

```text
world_id / branch_id / sim_year
```

Sort inside Parquet by:

```text
market_id / commodity_id / tick
```

Do not create a directory for every market × commodity combination. The archive is intended to remain efficient when the simulator expands from six exchanges to hundreds of settlements and many commodities.

## Local gates

- semantic analytics oracle: PASS;
- WDEX analytics request contract: PASS;
- consolidated reference suite through Sprint 15: PASS.

## Native gate still required

`tools/sprint15_duckdb_gate.py` must run in CI with DuckDB installed before ANA-001/ANA-002 are marked natively complete.
