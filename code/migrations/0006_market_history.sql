-- Hot/recent market history. Keep database-neutral PostgreSQL in the core migration set.
-- Timescale acceleration is optional and lives under code/sql/timescale.
CREATE TABLE IF NOT EXISTS market_tick (
    world_id UUID NOT NULL REFERENCES world(id),
    branch_id UUID NOT NULL REFERENCES branch(id),
    tick BIGINT NOT NULL,
    market_id TEXT NOT NULL,
    commodity_id TEXT NOT NULL,
    bid_mcp BIGINT NOT NULL,
    ask_mcp BIGINT NOT NULL,
    last_mcp BIGINT NOT NULL,
    volume_milli BIGINT NOT NULL DEFAULT 0,
    inventory_milli BIGINT NOT NULL,
    imports_milli BIGINT NOT NULL DEFAULT 0,
    exports_milli BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (branch_id, market_id, commodity_id, tick)
);
CREATE INDEX IF NOT EXISTS market_tick_lookup_idx ON market_tick (branch_id, market_id, commodity_id, tick DESC);

CREATE TABLE IF NOT EXISTS market_candle (
    branch_id UUID NOT NULL REFERENCES branch(id),
    market_id TEXT NOT NULL,
    commodity_id TEXT NOT NULL,
    interval_ticks INTEGER NOT NULL CHECK (interval_ticks > 0),
    bucket_start_tick BIGINT NOT NULL,
    open_mcp BIGINT NOT NULL,
    high_mcp BIGINT NOT NULL,
    low_mcp BIGINT NOT NULL,
    close_mcp BIGINT NOT NULL,
    volume_milli BIGINT NOT NULL,
    inventory_close_milli BIGINT NOT NULL,
    imports_milli BIGINT NOT NULL DEFAULT 0,
    exports_milli BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (branch_id, market_id, commodity_id, interval_ticks, bucket_start_tick)
);
CREATE INDEX IF NOT EXISTS market_candle_lookup_idx ON market_candle (branch_id, market_id, commodity_id, interval_ticks, bucket_start_tick DESC);
