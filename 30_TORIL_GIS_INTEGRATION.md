# Toril GIS Integration

## Status

**Accepted as canonical geography basis.**

The app now uses [[Toril GIS]] geographic coordinates as the canonical world-space model. The earlier campaign raster coordinate system remains supported as an overlay but is no longer authoritative geography.

## Why this changes the architecture

Toril GIS already provides a global, internally consistent geographic framework for Toril rather than a single Sword Coast image. The public export includes settlements, pathways, rivers, regions, coastlines/land/ocean and other layers. This is exactly the kind of geographic substrate the economic simulation needs.

## Coordinate policy

### Storage

Use **Toril GCS with the FRIA prime meridian**, matching the upstream GeoJSON exports without transformation.

Toril ellipsoid:

- semi-major axis: 6,410,000 m;
- semi-minor axis: 6,370,000 m;
- inverse flattening: 160.25.

### Display

Convert longitude to the lore-facing **Myth Drannor prime meridian** when showing coordinates to users.

```text
longitude_MD = normalize(longitude_FRIA + 53.48150148799°)
```

### Browser maps

MapLibre may treat Toril longitude/latitude numerically as ordinary map coordinates for rendering. All actual distance, travel and area calculations must use Toril-specific math, not Earth/WGS84 geodesics.

### Database

PostGIS custom SRID `910001` is reserved for Toril GCS / FRIA in this application.

## Upstream pin

Repository: `geospatial-grimoire/toril-gis`

Pinned commit used for initial integration:

`8f9bffae356ec32ec6f76ce788abf700d6e56c3b`

The GeoJSON export identifies itself as batch `2026-07-30_1`.

Updates are migrations, never silent replacements.

## Exchange coordinates recovered directly from Toril GIS

| Exchange | Settlement | Latitude | FRIA Longitude | Myth Drannor Longitude | Status |
|---|---|---:|---:|---:|---|
| [[WDEX]] | [[Waterdeep]] | 45.200070° | -73.632600° | -20.151099° | upstream exact |
| [[BGEX]] | [[Baldur's Gate]] | 37.675530° | -70.285140° | -16.803639° | upstream exact |
| [[CALEX]] | [[Calimport]] | 24.160140° | -68.695200° | -15.213699° | upstream exact |
| [[NWEX]] | [[Neverwinter]] | 49.305960° | -75.914910° | -22.433409° | upstream exact |
| [[LUSEX]] | [[Luskan]] | 51.344190° | -76.366620° | -22.885119° | upstream exact |

The SVG world export is 4000×2000 over the full `[-180,+180] × [-90,+90]` Toril GCS extent. Converting Waterdeep's SVG point reproduces the published Waterdeep coordinate, providing a useful independent check of the transformation.

## Athkatla caveat

The current public Toril GIS populated-place export does **not** contain a named [[Athkatla]] feature.

To keep the six-market prototype renderable, `ATHEX` currently has a **provisional mapping-only coordinate** derived from a secondary open Toril GeoJSON dataset transformed against five common Toril GIS anchors. Estimated anchor residual is about 0.2°; the app assigns a 30 km uncertainty and must never present this as a Toril GIS or canonical lore coordinate.

Replace it as soon as Toril GIS publishes or identifies Athkatla.

## Files

- `seed/toril_exchange_coordinates_v1.json`
- `seed/toril_route_geometries_v1.json`
- `code/web/map/toril-gcs.mjs`
- `code/web/map/toril-map-config.json`
- `code/migrations/0005_toril_gcs.sql`

## Next geographic imports

Priority layers:

1. populated places;
2. pathways/roads/trade corridors;
3. rivers;
4. coast/land/ocean boundaries;
5. named regions;
6. land cover and climate;
7. later, DEM/elevation.

These layers should be ingested into a separate source schema or build artifact first, then promoted into campaign geography only through versioned imports.
