# [[Historical Market Archive]]

## Purpose

The archive is the cold analytical history of [[WDEX]] and the wider Sword Coast market network. PostgreSQL/Timescale remain authoritative for current and recent state. Parquet is an immutable analytical projection, never an authoritative simulation input.

## Canonical source

`market_tick` rows are exported only after the source range is outside the configured hot-retention window and after the branch/tick range has been checkpointed.

## Partition strategy

Avoid a partition for every market/commodity. DuckDB recommends avoiding large numbers of small partitions; the target design therefore partitions by coarse fields and sorts within files.

```text
history/
  market_ticks/
    world_id=<world>/
      branch_id=<branch>/
        sim_year=<year>/
          data_<uuid>.parquet
```

Within each Parquet file rows are sorted by:

```text
market_id, commodity_id, tick
```

This keeps the number of folders bounded while allowing Parquet row-group statistics and projection/filter pushdown to prune most reads.

Target file size once the dataset is large enough: approximately 100 MB to 1 GB. Tiny campaigns may intentionally use smaller files.

## Required columns

- `world_id`
- `branch_id`
- `tick`
- `market_id`
- `commodity_id`
- `bid_mcp`
- `ask_mcp`
- `last_mcp`
- `volume_milli`
- `inventory_milli`
- `imports_milli`
- `exports_milli`
- `sim_year`

All simulation quantities remain signed/unsigned integer-compatible values. No floating-point money is introduced during archival.

## Shipment analytics

A second analytical projection, `shipment_fact`, is generated from completed/lost shipments. It stores purchase value, freight, loss status, arrival tick and a destination mark-to-market value. Until cargo-lot disposal accounting exists, route ROI is explicitly labelled **arrival mark-to-market ROI**, not realized trading profit.

## Immutability

Archive batches receive a manifest containing:

- archive schema version;
- simulation version;
- world and branch IDs;
- min/max tick;
- source row count;
- file list;
- SHA-256 for every file;
- source checkpoint/snapshot reference;
- export timestamp.

An existing batch is never silently rewritten. Corrections create a replacement batch and supersession record.
