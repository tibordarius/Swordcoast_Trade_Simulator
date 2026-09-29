# Sprint 12 — Map Foundation

## Status

**Campaign map coordinate space is grounded in the existing Obsidian vault and a local calibration tool is implemented. Geographic city coordinates remain intentionally unset until the actual raster is selected and clicked.**

## Existing campaign-map facts recovered

The vault note `Whale Campaign/Misc/SwordCoastMap.md` defines:

- image reference: `Sword-Coast-Map_HighRes.jpg`;
- canvas: `6600 × 10200 px`;
- center: `(3300, 5100)`;
- measurement unit: km;
- scale: `0.275 km/px`.

Nominal map extent at that scale is approximately:

- 1,815 km wide;
- 2,805 km high.

The actual JPG is referenced by the vault but is not contained in the uploaded campaign ZIPs, so settlement coordinates have **not** been fabricated.

## Delivered

### Map-space definition

`code/web/map/map-config.json`

The authoritative app stores raw custom-map coordinates instead of pretending Toril uses Earth latitude/longitude. PostGIS uses SRID 0 for this local geometry.

### Local calibration utility

`code/web/map/calibrate.html`

The user selects the existing Sword Coast map from the local Obsidian vault, then clicks:

1. [[Calimport]]
2. [[Athkatla]]
3. [[Baldur's Gate]]
4. [[Waterdeep]]
5. [[Neverwinter]]
6. [[Luskan]]

The tool exports raw pixels plus normalized coordinates. The image remains local to the browser.

### Database migration

`code/migrations/0004_map_space.sql`

Adds map-space metadata and links settlements/routes to the campaign map coordinate system.

## Important rule

The vault's `0.275 km/px` value is retained as campaign-map metadata, but straight-line pixel distance is not automatically treated as canonical sailing distance. Coastlines, currents, hazards and route geometry remain separate simulation inputs.

## Next map work

Once the six control points are captured:

- add settlement points to PostGIS;
- draw the first sea-route polylines;
- render the raster as a MapLibre custom map;
- then tile the 6600×10200 source into a PMTiles pyramid for efficient zooming.
