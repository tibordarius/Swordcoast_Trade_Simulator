# Sprint 13 — Live Logistics Map

## Status

**Reference layer complete; browser prototype complete. Final MapLibre/deck.gl rendering remains a later dependency integration step.**

## Delivered

### Toril geodesic math

`code/web/map/toril-gcs.mjs`

Implements:

- SVG-world coordinate conversion;
- FRIA ↔ Myth Drannor longitude conversion;
- Vincenty distance using Toril's ellipsoid;
- polyline length;
- distance-weighted shipment interpolation.

The published Waterdeep point resolves to the expected Toril coordinate and Waterdeep → Neverwinter direct distance is about 490 km.

### Route geometry builder

`tools/build_route_geometries.mjs`

Creates `seed/toril_route_geometries_v1.json`.

Current routes are **straight geodesic placeholders between port endpoints**. They are suitable for map animation and rough spatial reasoning, but not yet authoritative sailing lanes. Coastline avoidance, currents, navigable passages, hazards and route waypoints will replace these geometries.

### Client-side ship positioning

`code/web/map/live-logistics.mjs`

A shipment stores:

```text
route_id
departure_tick
eta_tick
```

The browser derives its current position. No continuous ship-position writes are required.

Benchmark on this runtime:

```text
10,000 shipment positions ≈ 30 ms
```

This supports very large visible fleets before GPU rendering becomes the bottleneck.

### MapLibre-ready data

The module emits GeoJSON FeatureCollections for:

- markets;
- routes;
- current shipment points.

It also calculates a bounded flow width from route trading volume.

### Browser demo

`code/web/map/live.html`

A dependency-free visual prototype renders:

- six exchanges;
- route thickness by trade volume;
- moving shipments;
- progress bars;
- provisional-route styling when [[Athkatla]] is involved.

This is deliberately not the final cartographic renderer. It validates state and animation behavior before adding MapLibre/deck.gl.

## Important result

The live map can update at visual frame rate while the economic simulation remains on its independent tick schedule. Geography therefore does not force higher simulation frequency.

## Next map step

Replace direct sea segments with navigable polylines using:

- Toril GIS coastline/land polygons;
- campaign sea lanes;
- route waypoints;
- piracy/storm zones;
- eventually currents and seasonal weather.
