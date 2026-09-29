BEGIN;

CREATE TABLE map_space (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  stable_key TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  width_px INTEGER NOT NULL CHECK (width_px > 0),
  height_px INTEGER NOT NULL CHECK (height_px > 0),
  meters_per_px BIGINT NOT NULL CHECK (meters_per_px > 0),
  axis_policy TEXT NOT NULL,
  source_reference TEXT,
  image_fingerprint TEXT
);

ALTER TABLE settlement
  ADD COLUMN map_space_id BIGINT REFERENCES map_space(id);

ALTER TABLE route
  ADD COLUMN map_space_id BIGINT REFERENCES map_space(id);

INSERT INTO map_space(stable_key,name,width_px,height_px,meters_per_px,axis_policy,source_reference)
VALUES(
  'sword_coast_campaign_map_v1',
  'Sword Coast Campaign Map',
  6600,
  10200,
  275,
  'raw image pixel coordinates; preserve orientation until control-point calibration is verified',
  'Whale Campaign/Misc/SwordCoastMap.md'
);

COMMIT;
