-- Historical spread between two exchanges.
WITH h AS (
  SELECT tick, market_id, commodity_id, last_mcp
  FROM read_parquet('history/market_ticks/**/*.parquet', hive_partitioning=true)
  WHERE commodity_id = 'GRAIN' AND market_id IN ('WDEX','BGEX')
), p AS (
  SELECT tick,
         max(last_mcp) FILTER (WHERE market_id='WDEX') AS wd,
         max(last_mcp) FILTER (WHERE market_id='BGEX') AS bg
  FROM h GROUP BY tick
)
SELECT tick, wd-bg AS spread_mcp FROM p WHERE wd IS NOT NULL AND bg IS NOT NULL ORDER BY tick;

-- Route/market volatility can be computed directly over archived candles without copying
-- history back into PostgreSQL.
