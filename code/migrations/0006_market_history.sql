BEGIN;

CREATE TABLE market_tick (
    world_id BIGINT NOT NULL REFERENCES world(id) ON DELETE CASCADE,
    branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
    tick BIGINT NOT NULL CHECK (tick >= 0),
    market_id BIGINT NOT NULL REFERENCES market(id),
    commodity_id BIGINT NOT NULL REFERENCES commodity(id),
    bid_mcp BIGINT NOT NULL CHECK (bid_mcp > 0),
    ask_mcp BIGINT NOT NULL CHECK (ask_mcp >= bid_mcp),
    last_mcp BIGINT NOT NULL CHECK (last_mcp > 0),
    volume_milli BIGINT NOT NULL DEFAULT 0 CHECK (volume_milli >= 0),
    inventory_milli BIGINT NOT NULL CHECK (inventory_milli >= 0),
    imports_milli BIGINT NOT NULL DEFAULT 0 CHECK (imports_milli >= 0),
    exports_milli BIGINT NOT NULL DEFAULT 0 CHECK (exports_milli >= 0),
    PRIMARY KEY (branch_id, market_id, commodity_id, tick)
);
CREATE INDEX market_tick_lookup_idx
  ON market_tick (branch_id, market_id, commodity_id, tick DESC);

CREATE TABLE market_candle (
    branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
    market_id BIGINT NOT NULL REFERENCES market(id),
    commodity_id BIGINT NOT NULL REFERENCES commodity(id),
    interval_ticks INTEGER NOT NULL CHECK (interval_ticks > 0),
    bucket_start_tick BIGINT NOT NULL CHECK (bucket_start_tick >= 0),
    open_mcp BIGINT NOT NULL CHECK (open_mcp > 0),
    high_mcp BIGINT NOT NULL CHECK (high_mcp >= open_mcp),
    low_mcp BIGINT NOT NULL CHECK (low_mcp > 0),
    close_mcp BIGINT NOT NULL CHECK (close_mcp > 0),
    volume_milli BIGINT NOT NULL CHECK (volume_milli >= 0),
    inventory_close_milli BIGINT NOT NULL CHECK (inventory_close_milli >= 0),
    imports_milli BIGINT NOT NULL DEFAULT 0 CHECK (imports_milli >= 0),
    exports_milli BIGINT NOT NULL DEFAULT 0 CHECK (exports_milli >= 0),
    PRIMARY KEY (branch_id, market_id, commodity_id, interval_ticks, bucket_start_tick),
    CHECK (high_mcp >= low_mcp),
    CHECK (high_mcp >= close_mcp),
    CHECK (low_mcp <= open_mcp),
    CHECK (low_mcp <= close_mcp)
);
CREATE INDEX market_candle_lookup_idx
  ON market_candle (branch_id, market_id, commodity_id, interval_ticks, bucket_start_tick DESC);

COMMIT;
