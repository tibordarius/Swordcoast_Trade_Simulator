-- Replay watermark and authoritative event-log head.
-- The branch head makes truncating the tail of an otherwise valid hash chain detectable.
ALTER TABLE branch
  ADD COLUMN IF NOT EXISTS last_event_sequence BIGINT NOT NULL DEFAULT 0
  CHECK (last_event_sequence >= 0);

ALTER TABLE branch
  ADD COLUMN IF NOT EXISTS last_event_hash TEXT NOT NULL DEFAULT 'GENESIS';

ALTER TABLE snapshot
  ADD COLUMN IF NOT EXISTS event_sequence BIGINT NOT NULL DEFAULT 0
  CHECK (event_sequence >= 0);

CREATE INDEX IF NOT EXISTS snapshot_restore_idx
  ON snapshot (branch_id, simulation_version, tick DESC, id DESC);
