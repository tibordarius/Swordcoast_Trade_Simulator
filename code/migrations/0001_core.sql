BEGIN;

CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TYPE provenance_class AS ENUM (
  'fr_canon',
  'campaign_canon',
  'inference',
  'simulation_generated'
);

CREATE TYPE branch_status AS ENUM ('active', 'paused', 'archived');
CREATE TYPE shipment_status AS ENUM ('planned', 'reserved', 'in_transit', 'arrived', 'lost', 'cancelled');
CREATE TYPE order_side AS ENUM ('buy', 'sell');
CREATE TYPE order_type AS ENUM ('market', 'limit');
CREATE TYPE order_status AS ENUM ('open', 'partial', 'filled', 'cancelled', 'rejected');

CREATE TABLE provenance (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  classification provenance_class NOT NULL,
  source_title TEXT,
  source_reference TEXT,
  page_or_location TEXT,
  note TEXT,
  confidence SMALLINT CHECK (confidence BETWEEN 0 AND 100)
);

CREATE TABLE world (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  seed BIGINT NOT NULL,
  simulation_version TEXT NOT NULL,
  current_tick BIGINT NOT NULL DEFAULT 0 CHECK (current_tick >= 0),
  tick_minutes SMALLINT NOT NULL DEFAULT 5 CHECK (tick_minutes > 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE branch (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  world_id BIGINT NOT NULL REFERENCES world(id) ON DELETE CASCADE,
  parent_branch_id BIGINT REFERENCES branch(id),
  parent_snapshot_id BIGINT,
  name TEXT NOT NULL,
  status branch_status NOT NULL DEFAULT 'active',
  canonical BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE (world_id, name)
);

CREATE UNIQUE INDEX branch_one_canonical_per_world
  ON branch(world_id)
  WHERE canonical;

CREATE TABLE region (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  provenance_id BIGINT REFERENCES provenance(id),
  geom geometry(MultiPolygon, 0)
);

CREATE TABLE settlement (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  canonical_name TEXT NOT NULL,
  settlement_type TEXT NOT NULL,
  region_id BIGINT REFERENCES region(id),
  population_reference BIGINT CHECK (population_reference IS NULL OR population_reference >= 0),
  port_capability BOOLEAN NOT NULL DEFAULT FALSE,
  river_capability BOOLEAN NOT NULL DEFAULT FALSE,
  road_capability BOOLEAN NOT NULL DEFAULT TRUE,
  provenance_id BIGINT REFERENCES provenance(id),
  geom geometry(Point, 0) NOT NULL
);
CREATE INDEX settlement_geom_gix ON settlement USING GIST (geom);

CREATE TABLE commodity (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  family TEXT NOT NULL,
  base_unit TEXT NOT NULL,
  quantity_scale INTEGER NOT NULL DEFAULT 1000 CHECK (quantity_scale > 0),
  mass_mg_per_milli_unit BIGINT CHECK (mass_mg_per_milli_unit IS NULL OR mass_mg_per_milli_unit >= 0),
  volume_mm3_per_milli_unit BIGINT CHECK (volume_mm3_per_milli_unit IS NULL OR volume_mm3_per_milli_unit >= 0),
  perishability_class TEXT,
  legality_class TEXT NOT NULL DEFAULT 'legal',
  reference_price_mcp BIGINT NOT NULL CHECK (reference_price_mcp > 0),
  provenance_id BIGINT REFERENCES provenance(id)
);

CREATE TABLE market (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  settlement_id BIGINT NOT NULL REFERENCES settlement(id),
  exchange_code TEXT UNIQUE,
  market_type TEXT NOT NULL DEFAULT 'spot',
  liquidity_tier SMALLINT NOT NULL CHECK (liquidity_tier BETWEEN 1 AND 5)
);

CREATE TABLE market_inventory (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  market_id BIGINT NOT NULL REFERENCES market(id),
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  on_hand_milli BIGINT NOT NULL DEFAULT 0 CHECK (on_hand_milli >= 0),
  reserved_milli BIGINT NOT NULL DEFAULT 0 CHECK (reserved_milli >= 0),
  target_reserve_milli BIGINT NOT NULL DEFAULT 0 CHECK (target_reserve_milli >= 0),
  incoming_committed_milli BIGINT NOT NULL DEFAULT 0 CHECK (incoming_committed_milli >= 0),
  outgoing_committed_milli BIGINT NOT NULL DEFAULT 0 CHECK (outgoing_committed_milli >= 0),
  unmet_demand_milli BIGINT NOT NULL DEFAULT 0 CHECK (unmet_demand_milli >= 0),
  updated_tick BIGINT NOT NULL CHECK (updated_tick >= 0),
  PRIMARY KEY (branch_id, market_id, commodity_id),
  CHECK (reserved_milli <= on_hand_milli)
);

CREATE TABLE market_state (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  market_id BIGINT NOT NULL REFERENCES market(id),
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  fundamental_mcp BIGINT NOT NULL CHECK (fundamental_mcp > 0),
  bid_mcp BIGINT NOT NULL CHECK (bid_mcp > 0),
  ask_mcp BIGINT NOT NULL CHECK (ask_mcp >= bid_mcp),
  reserve_bps INTEGER NOT NULL,
  unmet_bps INTEGER NOT NULL,
  incoming_bps INTEGER NOT NULL,
  risk_bps INTEGER NOT NULL,
  total_multiplier_bps INTEGER NOT NULL,
  updated_tick BIGINT NOT NULL CHECK (updated_tick >= 0),
  PRIMARY KEY (branch_id, market_id, commodity_id)
);

CREATE TABLE route (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  route_type TEXT NOT NULL,
  origin_settlement_id BIGINT NOT NULL REFERENCES settlement(id),
  destination_settlement_id BIGINT NOT NULL REFERENCES settlement(id),
  distance_m BIGINT NOT NULL CHECK (distance_m > 0),
  base_travel_ticks BIGINT NOT NULL CHECK (base_travel_ticks > 0),
  capacity_milli BIGINT NOT NULL CHECK (capacity_milli > 0),
  base_cost_mcp_per_milli BIGINT NOT NULL CHECK (base_cost_mcp_per_milli >= 0),
  base_risk_bps INTEGER NOT NULL DEFAULT 0 CHECK (base_risk_bps BETWEEN 0 AND 10000),
  active BOOLEAN NOT NULL DEFAULT TRUE,
  geom geometry(LineString, 0) NOT NULL
);
CREATE INDEX route_geom_gix ON route USING GIST (geom);

CREATE TABLE trader (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  trader_type TEXT NOT NULL,
  faction_key TEXT,
  risk_tolerance_bps INTEGER NOT NULL DEFAULT 5000 CHECK (risk_tolerance_bps BETWEEN 0 AND 10000),
  information_quality_bps INTEGER NOT NULL DEFAULT 5000 CHECK (information_quality_bps BETWEEN 0 AND 10000),
  strategy_profile JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE trader_account (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  trader_id BIGINT NOT NULL REFERENCES trader(id),
  cash_mcp BIGINT NOT NULL CHECK (cash_mcp >= 0),
  updated_tick BIGINT NOT NULL CHECK (updated_tick >= 0),
  PRIMARY KEY (branch_id, trader_id)
);

CREATE TABLE shipment (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  owner_trader_id BIGINT REFERENCES trader(id),
  origin_market_id BIGINT NOT NULL REFERENCES market(id),
  destination_market_id BIGINT NOT NULL REFERENCES market(id),
  route_id BIGINT NOT NULL REFERENCES route(id),
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  quantity_milli BIGINT NOT NULL CHECK (quantity_milli > 0),
  purchase_value_mcp BIGINT NOT NULL CHECK (purchase_value_mcp >= 0),
  transport_cost_mcp BIGINT NOT NULL DEFAULT 0 CHECK (transport_cost_mcp >= 0),
  departure_tick BIGINT NOT NULL CHECK (departure_tick >= 0),
  eta_tick BIGINT NOT NULL CHECK (eta_tick > departure_tick),
  status shipment_status NOT NULL,
  loss_reason TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX shipment_branch_eta_idx ON shipment(branch_id, eta_tick) WHERE status = 'in_transit';

CREATE TABLE market_order (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  actor_key TEXT NOT NULL,
  market_id BIGINT NOT NULL REFERENCES market(id),
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  side order_side NOT NULL,
  order_type order_type NOT NULL,
  quantity_milli BIGINT NOT NULL CHECK (quantity_milli > 0),
  remaining_milli BIGINT NOT NULL CHECK (remaining_milli >= 0 AND remaining_milli <= quantity_milli),
  limit_price_mcp BIGINT CHECK (limit_price_mcp IS NULL OR limit_price_mcp > 0),
  created_tick BIGINT NOT NULL CHECK (created_tick >= 0),
  status order_status NOT NULL
);

CREATE TABLE trade_execution (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  order_id BIGINT REFERENCES market_order(id),
  market_id BIGINT NOT NULL REFERENCES market(id),
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  quantity_milli BIGINT NOT NULL CHECK (quantity_milli > 0),
  price_mcp BIGINT NOT NULL CHECK (price_mcp > 0),
  notional_mcp BIGINT NOT NULL CHECK (notional_mcp > 0),
  buyer_key TEXT NOT NULL,
  seller_key TEXT NOT NULL,
  tick BIGINT NOT NULL CHECK (tick >= 0)
);

CREATE TABLE input_event_log (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  sequence BIGINT NOT NULL CHECK (sequence > 0),
  tick_received BIGINT NOT NULL CHECK (tick_received >= 0),
  actor TEXT NOT NULL,
  command_type TEXT NOT NULL,
  payload JSONB NOT NULL,
  schema_version INTEGER NOT NULL CHECK (schema_version > 0),
  event_hash TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (branch_id, sequence),
  UNIQUE (branch_id, event_hash)
);

CREATE TABLE snapshot (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  tick BIGINT NOT NULL CHECK (tick >= 0),
  simulation_version TEXT NOT NULL,
  state_format TEXT NOT NULL,
  state_uri TEXT,
  state_bytes BYTEA,
  checksum TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CHECK ((state_uri IS NOT NULL)::int + (state_bytes IS NOT NULL)::int = 1),
  UNIQUE (branch_id, tick, checksum)
);

ALTER TABLE branch
  ADD CONSTRAINT branch_parent_snapshot_fk
  FOREIGN KEY (parent_snapshot_id) REFERENCES snapshot(id);

COMMIT;
