BEGIN;

INSERT INTO spatial_ref_sys (srid, auth_name, auth_srid, srtext, proj4text)
VALUES (
  910001,
  NULL,
  NULL,
  'GEOGCRS["Toril GCS",DATUM["Toril",ELLIPSOID["Toril",6410000,160.25,LENGTHUNIT["metre",1]]],PRIMEM["FRIA",0,ANGLEUNIT["degree",0.0174532925199433]],CS[ellipsoidal,2],AXIS["latitude (Lat)",north,ORDER[1],ANGLEUNIT["degree",0.0174532925199433]],AXIS["longitude (Lon)",east,ORDER[2],ANGLEUNIT["degree",0.0174532925199433]],USAGE[SCOPE["Web mapping and visualisation of Toril."],AREA["World."],BBOX[-90,-180,90,180]]]',
  '+proj=longlat +a=6410000 +b=6370000 +no_defs +type=crs'
)
ON CONFLICT (srid) DO NOTHING;

ALTER TABLE settlement
  ADD COLUMN toril_geom geometry(Point, 910001),
  ADD COLUMN toril_coordinate_status TEXT,
  ADD COLUMN toril_source_uuid TEXT,
  ADD COLUMN toril_source_commit TEXT,
  ADD COLUMN toril_uncertainty_m BIGINT CHECK (toril_uncertainty_m IS NULL OR toril_uncertainty_m >= 0);

CREATE INDEX settlement_toril_geom_gix ON settlement USING GIST (toril_geom);
CREATE UNIQUE INDEX settlement_toril_source_uuid_uq
  ON settlement(toril_source_uuid)
  WHERE toril_source_uuid IS NOT NULL;

ALTER TABLE route
  ADD COLUMN toril_geom geometry(LineString, 910001),
  ADD COLUMN geometry_status TEXT,
  ADD COLUMN coordinate_uncertainty_m BIGINT CHECK (coordinate_uncertainty_m IS NULL OR coordinate_uncertainty_m >= 0),
  ADD COLUMN distance_source TEXT;

CREATE INDEX route_toril_geom_gix ON route USING GIST (toril_geom);

COMMENT ON COLUMN settlement.toril_geom IS
  'Canonical Toril GCS location using the FRIA prime meridian. Display longitude may be shifted to Myth Drannor in the client.';
COMMENT ON COLUMN route.toril_geom IS
  'Toril GCS route polyline. Route distance_m remains authoritative for simulation once geometry_status is navigable/certified.';

COMMIT;
