BEGIN;

CREATE OR REPLACE FUNCTION reject_mutation_on_append_only()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  RAISE EXCEPTION 'table % is append-only', TG_TABLE_NAME;
END;
$$;

CREATE TRIGGER input_event_log_no_update
BEFORE UPDATE ON input_event_log
FOR EACH ROW EXECUTE FUNCTION reject_mutation_on_append_only();

CREATE TRIGGER input_event_log_no_delete
BEFORE DELETE ON input_event_log
FOR EACH ROW EXECUTE FUNCTION reject_mutation_on_append_only();

CREATE TRIGGER trade_execution_no_update
BEFORE UPDATE ON trade_execution
FOR EACH ROW EXECUTE FUNCTION reject_mutation_on_append_only();

CREATE TRIGGER trade_execution_no_delete
BEFORE DELETE ON trade_execution
FOR EACH ROW EXECUTE FUNCTION reject_mutation_on_append_only();

COMMIT;
