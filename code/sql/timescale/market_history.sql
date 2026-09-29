-- OPTIONAL deployment accelerator. Run only on a PostgreSQL instance with TimescaleDB.
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Convert the hot tick table into a hypertable. Tick is authoritative simulation time.
SELECT create_hypertable('market_tick', by_range('tick', 28800), if_not_exists => TRUE, migrate_data => TRUE);

-- Recommended retention/columnstore policies depend on campaign speed and are intentionally
-- not hard-coded in the schema. The application may keep recent raw ticks hot and archive
-- older rows to Parquet before dropping them from PostgreSQL.
