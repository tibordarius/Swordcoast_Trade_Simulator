-- Replay watermark for binary world snapshots.
-- The snapshot captures authoritative state after all input events up to event_sequence.
ALTER TABLE snapshot
  ADD COLUMN IF NOT EXISTS event_sequence BIGINT NOT NULL DEFAULT 0
  CHECK (event_sequence >= 0);

CREATE INDEX IF NOT EXISTS snapshot_restore_idx
  ON snapshot (branch_id, simulation_version, tick DESC, id DESC);
