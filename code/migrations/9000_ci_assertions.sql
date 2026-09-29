DO $$
DECLARE
  missing_count integer;
  bad_numeric_count integer;
BEGIN
  SELECT count(*) INTO missing_count
  FROM (VALUES
    ('world'), ('branch'), ('settlement'), ('commodity'), ('market'),
    ('market_inventory'), ('market_state'), ('route'), ('shipment'),
    ('input_event_log'), ('snapshot')
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
END $$;
