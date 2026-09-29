-- Sprint 15 cold-history export.
-- Expected setup: DuckDB has PostgreSQL attached as `pg`, or `market_tick_source`
-- has been populated from an Arrow/CSV stream with the same schema.
-- Coarse partitioning is intentional; do not partition by market + commodity.

CREATE OR REPLACE TEMP VIEW archive_market_tick AS
SELECT
  world_id::VARCHAR AS world_id,
  branch_id::VARCHAR AS branch_id,
  tick::BIGINT AS tick,
  market_id::VARCHAR AS market_id,
  commodity_id::VARCHAR AS commodity_id,
  bid_mcp::BIGINT AS bid_mcp,
  ask_mcp::BIGINT AS ask_mcp,
  last_mcp::BIGINT AS last_mcp,
  volume_milli::BIGINT AS volume_milli,
  inventory_milli::BIGINT AS inventory_milli,
  imports_milli::BIGINT AS imports_milli,
  exports_milli::BIGINT AS exports_milli,
  floor(tick / 105120)::BIGINT AS sim_year
FROM market_tick_source
ORDER BY market_id, commodity_id, tick;

COPY archive_market_tick
TO 'history/market_ticks'
(
  FORMAT PARQUET,
  COMPRESSION ZSTD,
  PARTITION_BY (world_id, branch_id, sim_year),
  FILENAME_PATTERN 'data_{uuid}'
);
