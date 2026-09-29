DO $$
DECLARE
  missing_count integer;
  bad_numeric_count integer;
BEGIN
  SELECT count(*) INTO missing_count
  FROM (VALUES
    ('world'), ('branch'), ('settlement'), ('commodity'), ('market'),
    ('market_inventory'), ('market_state'), ('route'), ('shipment'),
    ('input_event_log'), ('snapshot'), ('market_tick'), ('market_candle'),
    ('production_recipe'), ('production_site')
  ) AS required(name)
  LEFT JOIN information_schema.tables t
    ON t.table_schema = 'public' AND t.table_name = required.name
  WHERE t.table_name IS NULL;

  IF missing_count <> 0 THEN
    RAISE EXCEPTION 'missing required tables: %', missing_count;
  END IF;

  SELECT count(*) INTO bad_numeric_count
  FROM information_schema.columns
  WHERE table_schema = 'public'
    AND data_type IN ('real', 'double precision');

  IF bad_numeric_count <> 0 THEN
    RAISE EXCEPTION 'floating-point economic columns found: %', bad_numeric_count;
  END IF;

  IF NOT EXISTS (
    SELECT 1 FROM pg_extension WHERE extname = 'postgis'
  ) THEN
    RAISE EXCEPTION 'PostGIS extension not installed';
  END IF;

  IF EXISTS (
    SELECT 1
    FROM information_schema.columns
    WHERE table_schema='public'
      AND table_name IN ('market_tick','market_candle')
      AND column_name IN ('world_id','branch_id')
      AND data_type <> 'bigint'
  ) THEN
    RAISE EXCEPTION 'market-history identity columns must use BIGINT';
  END IF;
END $$;
