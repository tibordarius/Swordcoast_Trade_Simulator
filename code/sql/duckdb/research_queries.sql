-- Canonical research-query examples for Sprint 15.
-- Bind :root, :branch_id, :commodity_id, etc. in application code instead of
-- interpolating untrusted strings.

-- 1. Historical spread between two exchanges.
WITH h AS (
  SELECT tick, market_id, last_mcp
  FROM read_parquet('history/market_ticks/**/*.parquet', hive_partitioning=true)
  WHERE branch_id = 'MAIN'
    AND commodity_id = 'GRAIN'
    AND market_id IN ('WDEX','BGEX')
), p AS (
  SELECT tick,
         max(last_mcp) FILTER (WHERE market_id='WDEX') AS wd,
         max(last_mcp) FILTER (WHERE market_id='BGEX') AS bg
  FROM h GROUP BY tick
)
SELECT tick, wd, bg, wd-bg AS spread_mcp
FROM p
WHERE wd IS NOT NULL AND bg IS NOT NULL
ORDER BY tick;

-- 2. Realized market volume by exchange.
SELECT market_id, commodity_id, sum(volume_milli) AS volume_milli
FROM read_parquet('history/market_ticks/**/*.parquet', hive_partitioning=true)
WHERE branch_id='MAIN'
GROUP BY market_id, commodity_id
ORDER BY volume_milli DESC;

-- 3. Integer-friendly volatility proxy: mean absolute price movement in basis points.
WITH ordered AS (
  SELECT market_id, commodity_id, tick, last_mcp,
         lag(last_mcp) OVER (PARTITION BY market_id, commodity_id ORDER BY tick) AS prev_mcp
  FROM read_parquet('history/market_ticks/**/*.parquet', hive_partitioning=true)
  WHERE branch_id='MAIN'
), moves AS (
  SELECT market_id, commodity_id,
         abs(last_mcp-prev_mcp) * 10000 // nullif(prev_mcp,0) AS abs_move_bps
  FROM ordered WHERE prev_mcp IS NOT NULL
)
SELECT market_id, commodity_id,
       avg(abs_move_bps)::DECIMAL(18,2) AS mean_abs_move_bps,
       max(abs_move_bps) AS max_abs_move_bps
FROM moves GROUP BY market_id, commodity_id;

-- 4. Market share by executed notional. This query is against PostgreSQL/current
-- trade-execution history until trade executions receive their own Parquet fact.
-- SELECT market_id, buyer_key, sum(notional_mcp) ... GROUP BY ...

-- 5. Route ROI uses shipment_fact once exported. It is ARRIVAL MARK-TO-MARKET ROI,
-- not realized disposal profit until cargo-lot sale attribution exists.
-- SELECT route_id,
--        sum(destination_mark_value_mcp - purchase_value_mcp - transport_cost_mcp)
--          * 10000 // nullif(sum(purchase_value_mcp + transport_cost_mcp),0) AS roi_bps
-- FROM read_parquet('history/shipment_fact/**/*.parquet', hive_partitioning=true)
-- WHERE status='arrived'
-- GROUP BY route_id;
