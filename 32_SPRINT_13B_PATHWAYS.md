# Sprint 13b — Published Toril Pathway Network

**Status:** Reference complete  
**Date:** 2026-09-29

## Goal

Replace hand-authored straight overland links with the version-pinned road/travel-corridor geometry published by Toril GIS wherever that geometry exists.

## Upstream

- Repository: `geospatial-grimoire/toril-gis`
- Commit: `8f9bffae356ec32ec6f76ce788abf700d6e56c3b`
- Layer: `srf_civ_pathways_ln`
- CRS: Toril GCS / FRIA

## Findings

The layer contains:

- 324 `Primary Road` features;
- 5 `Underwater Tunnel` features;
- 329 total pathways.

Within the initial Sword Coast economic bounding box there are 42 intersecting pathway features and 666 published vertices.

Most published paths are unnamed or carry only their UUID as a title. Their geometry is nevertheless useful and remains traceable to an upstream UUID.

## Routing implementation

`tools/build_transport_graph.mjs`:

1. reads the clipped pathway GeoJSON produced by the pinned Toril GIS importer;
2. converts every published road vertex into a graph node;
3. converts every consecutive polyline segment into a bidirectional transport edge;
4. calculates edge length using Toril's ellipsoid, not WGS84;
5. retains upstream feature UUID and class on every edge;
6. supports nearest-node settlement attachment and Dijkstra shortest-path routing.

## Northern validation corridor

Published Toril GIS geometry attaches exactly to:

- [[Luskan]];
- [[Neverwinter]];
- [[Waterdeep]].

Reference results:

| Corridor | Published-road distance |
|---|---:|
| [[Luskan]] → [[Neverwinter]] | ~233 km |
| [[Neverwinter]] → [[Waterdeep]] | ~529 km |

These are not straight-line distances. They follow the actual road vertices in the pinned GIS layer.

## Important distinction

The overland graph is now increasingly source-derived.

The maritime graph is **not**.

Existing direct sea polylines are still routing placeholders and must remain marked as such until we build a navigation graph from ports, coastline geometry, hazards, currents/winds and campaign sea lanes.

## Exit gate

- published pathways can be imported reproducibly;
- a routing graph can be generated from them;
- settlement-to-road attachment works;
- Toril-distance routing passes reference tests;
- route provenance survives graph generation.
