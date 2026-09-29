BEGIN;

CREATE TABLE warehouse (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  market_id BIGINT NOT NULL REFERENCES market(id),
  owner_trader_id BIGINT REFERENCES trader(id),
  name TEXT NOT NULL,
  max_mass_mg BIGINT NOT NULL CHECK (max_mass_mg > 0),
  max_volume_mm3 BIGINT NOT NULL CHECK (max_volume_mm3 > 0)
);

CREATE TABLE warehouse_position (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  warehouse_id BIGINT NOT NULL REFERENCES warehouse(id) ON DELETE CASCADE,
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  quantity_milli BIGINT NOT NULL DEFAULT 0 CHECK (quantity_milli >= 0),
  reserved_milli BIGINT NOT NULL DEFAULT 0 CHECK (reserved_milli >= 0),
  updated_tick BIGINT NOT NULL CHECK (updated_tick >= 0),
  PRIMARY KEY (branch_id, warehouse_id, commodity_id),
  CHECK (reserved_milli <= quantity_milli)
);

CREATE TABLE vessel (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  owner_trader_id BIGINT REFERENCES trader(id),
  name TEXT NOT NULL,
  max_mass_mg BIGINT NOT NULL CHECK (max_mass_mg > 0),
  max_volume_mm3 BIGINT NOT NULL CHECK (max_volume_mm3 > 0)
);

CREATE TABLE vessel_cargo (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  vessel_id BIGINT NOT NULL REFERENCES vessel(id) ON DELETE CASCADE,
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  quantity_milli BIGINT NOT NULL DEFAULT 0 CHECK (quantity_milli >= 0),
  reserved_milli BIGINT NOT NULL DEFAULT 0 CHECK (reserved_milli >= 0),
  updated_tick BIGINT NOT NULL CHECK (updated_tick >= 0),
  PRIMARY KEY (branch_id, vessel_id, commodity_id),
  CHECK (reserved_milli <= quantity_milli)
);

ALTER TABLE market_order
  ADD COLUMN reserved_cash_mcp BIGINT NOT NULL DEFAULT 0 CHECK (reserved_cash_mcp >= 0);

COMMIT;
