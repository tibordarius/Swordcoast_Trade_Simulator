BEGIN;

CREATE TABLE production_recipe (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  site_type TEXT NOT NULL,
  cycle_ticks BIGINT NOT NULL CHECK (cycle_ticks > 0),
  provenance_id BIGINT REFERENCES provenance(id)
);

CREATE TABLE recipe_input (
  recipe_id BIGINT NOT NULL REFERENCES production_recipe(id) ON DELETE CASCADE,
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  quantity_milli BIGINT NOT NULL CHECK (quantity_milli > 0),
  PRIMARY KEY (recipe_id, commodity_id)
);

CREATE TABLE recipe_output (
  recipe_id BIGINT NOT NULL REFERENCES production_recipe(id) ON DELETE CASCADE,
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  quantity_milli BIGINT NOT NULL CHECK (quantity_milli > 0),
  output_role TEXT NOT NULL DEFAULT 'primary' CHECK (output_role IN ('primary','byproduct')),
  PRIMARY KEY (recipe_id, commodity_id)
);

CREATE TABLE production_site (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  site_type TEXT NOT NULL,
  settlement_id BIGINT REFERENCES settlement(id),
  region_id BIGINT REFERENCES region(id),
  owner_trader_id BIGINT REFERENCES trader(id),
  parallel_capacity INTEGER NOT NULL DEFAULT 1 CHECK (parallel_capacity > 0),
  efficiency_bps INTEGER NOT NULL DEFAULT 10000 CHECK (efficiency_bps BETWEEN 1 AND 20000),
  provenance_id BIGINT REFERENCES provenance(id),
  CHECK (settlement_id IS NOT NULL OR region_id IS NOT NULL)
);

CREATE TABLE site_recipe (
  site_id BIGINT NOT NULL REFERENCES production_site(id) ON DELETE CASCADE,
  recipe_id BIGINT NOT NULL REFERENCES production_recipe(id) ON DELETE CASCADE,
  priority SMALLINT NOT NULL DEFAULT 100,
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  PRIMARY KEY (site_id, recipe_id)
);

CREATE TABLE site_inventory (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  site_id BIGINT NOT NULL REFERENCES production_site(id) ON DELETE CASCADE,
  commodity_id BIGINT NOT NULL REFERENCES commodity(id),
  on_hand_milli BIGINT NOT NULL DEFAULT 0 CHECK (on_hand_milli >= 0),
  reserved_milli BIGINT NOT NULL DEFAULT 0 CHECK (reserved_milli >= 0),
  updated_tick BIGINT NOT NULL CHECK (updated_tick >= 0),
  PRIMARY KEY (branch_id, site_id, commodity_id),
  CHECK (reserved_milli <= on_hand_milli)
);

CREATE TABLE production_job (
  branch_id BIGINT NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
  site_id BIGINT NOT NULL REFERENCES production_site(id) ON DELETE CASCADE,
  line_no INTEGER NOT NULL CHECK (line_no > 0),
  recipe_id BIGINT NOT NULL REFERENCES production_recipe(id),
  batches BIGINT NOT NULL DEFAULT 1 CHECK (batches > 0),
  started_tick BIGINT NOT NULL CHECK (started_tick >= 0),
  completion_tick BIGINT NOT NULL CHECK (completion_tick > started_tick),
  PRIMARY KEY (branch_id, site_id, line_no)
);
CREATE INDEX production_job_completion_idx
  ON production_job(branch_id, completion_tick);

COMMIT;
