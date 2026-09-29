# Research

## Research Principle

Separate **what the setting explicitly says** from **what the simulation needs to infer**.

Every imported fact should be labelled:

- **FR Canon** — explicitly supported by a Forgotten Realms source.
- **Campaign Canon** — established in the Whale Campaign.
- **Inference** — economically/geographically reasoned from supported facts.
- **Simulation Generated** — emergent current-world state.

## User-Supplied Reference Set

- Reddit: `https://www.reddit.com/r/DnDBehindTheScreen/comments/1i1c2o4/trade_and_economics_in_the_sword_coast_region_of/`
- Forgotten Realms Wiki, Mercantile Company: `https://forgottenrealms.fandom.com/wiki/Mercantile_company`
- LinkedIn commodity trading article: `https://www.linkedin.com/pulse/commodity-trading-across-faer%C3%BBn-how-waterdeep-company-murray-fife-3yube/`
- SJ Games forum thread: `https://forums.sjgames.com/showthread.php?t=42351`
- The Alexandrian, The Trade Way: `https://thealexandrian.net/wordpress/44297/roleplaying-games/forgotten-realms-the-trade-way`
- Facebook discussion: `https://www.facebook.com/groups/392678574938393/posts/976157276590517/`

## Source Priority

1. Forgotten Realms books/sourcebooks and the project's converted FR corpus.
2. Explicit Whale Campaign canon.
3. Lore wikis where the underlying source citation can be checked.
4. High-quality route/economic reconstruction such as The Alexandrian.
5. Community historical-economic reasoning.
6. Unsourced extrapolation only when needed, clearly labelled.

## Current Economic Hypotheses

### Northern resource gradient

The northern Sword Coast should tend to export natural resources such as timber, metals, hides/furs and selected crafts while importing food, manufactured goods and luxuries.

### Waterdeep as processor/redistributor

[[Waterdeep]] should consume large quantities of raw materials and food while producing/refining a broad range of finished goods. It is more useful as a liquid hub than as simply the market with the highest prices.

### Southern agricultural/luxury flows

[[Amn]], [[Tethyr]] and [[Calimshan]] are candidates for substantial food, wine, textiles, luxury and exotic-goods flows toward northern cities, depending on the underlying source material.

### Trade routes are choices

Merchants should choose among sea, road, river and clandestine routes based on time, cost, capacity, tariffs and risk rather than pure geographic distance.

### Whale products matter economically

The campaign already makes whaling and whale politics important. [[Whale Oil]], [[Ambergris]], bone/baleen-equivalent materials, and related substitutes should form meaningful commodity chains rather than decorative lore.

## Research Work Packages

### R01 — Canonical settlement import/export extraction

For each settlement/region:

- imports;
- exports;
- industries;
- agricultural products;
- mines/resources;
- population estimate and period;
- ports/roads/rivers;
- political/tariff constraints;
- source/page.

### R02 — Trade route graph

Extract and reconcile:

- named roads;
- river navigability;
- sea lanes;
- passes;
- seasonal closures;
- dangerous regions;
- customs/tolls;
- pirate areas.

### R03 — Commodity ontology

Determine a practical hierarchy:

`raw resource -> processed material -> finished commodity -> luxury/specialist commodity`

Avoid excessive granularity until it produces gameplay value.

### R04 — Historical pricing analogues

Use historical maritime/Hanseatic/East India trade only as an economic calibration tool, not as lore canon.

Research:

- freight share of cargo value;
- warehouse costs;
- loss rates;
- bulk vs luxury margins;
- seasonal grain pricing;
- route capacity;
- insurance/risk premiums.

### R05 — Market microstructure

Research what elements actually improve gameplay:

- spot market;
- bid/ask spread;
- market depth;
- slippage;
- warehouse receipt;
- forward delivery contract;
- charter contract;
- index construction.

Avoid derivatives until the physical market works.

## Technical Research Queue

- Benchmark Rust simulation representations: AoS vs SoA.
- Benchmark PostgreSQL partitions vs Timescale for expected tick volume.
- Benchmark Arrow IPC vs MessagePack/JSON for live market deltas.
- Benchmark DuckDB-Wasm with 100 MB, 500 MB and 1 GB representative Parquet history.
- Test PMTiles with custom fantasy coordinates/map raster/vector sources.
- Compare pgRouting against precomputed/in-memory Rust route matrices.
- Investigate whether H3 is appropriate after Toril georeferencing; do not force Earth assumptions onto the map.

## Open Research Questions

- What Forgotten Realms year/edition is the campaign economy anchored to?
- What is the economic scale of magic in normal trade versus rare specialist trade?
- What population estimates should be canonical for simulation purposes?
- How much food reserve should large cities maintain?
- How quickly does price information travel without magical communication?
- Which factions have magical communications that materially improve information latency?
- How common are teleportation circles for commercial freight versus people/information?
- How should intelligent undersea societies interact with surface logistics?

---

# Toril GIS Research — 2026-09-29

## Primary source

Geospatial Grimoire's Toril GIS provides a custom global coordinate system and a public QGIS/GeoJSON/SVG geographic dataset for Toril.

Important technical facts adopted by the project:

- Toril GCS ellipsoid: `a=6,410,000 m`, `b=6,370,000 m`, inverse flattening `160.25`;
- raw/public GeoJSON uses the FRIA prime meridian;
- the user-facing/lore-friendly coordinate system shifts longitude to the Myth Drannor prime meridian;
- the public GeoJSON batch `2026-07-30_1` contains 29 layers, including 1,030 populated places, 329 pathways and 1,195 rivers;
- the public repository is explicitly work in progress and regional detail may be incomplete or approximate;
- browser Toril Explorer uses MapLibre while applying Toril-specific geodesic calculations rather than Earth measurements.

Project upstream pin: `geospatial-grimoire/toril-gis@8f9bffae356ec32ec6f76ce788abf700d6e56c3b`.

## Secondary coordinate fallback

`BeauNouvelle/toril-geojson` was inspected only to create a temporary mapping coordinate for [[Athkatla]], which is absent by name from the current Toril GIS populated-place export. This value is explicitly inference, not canon and not Toril GIS data.


## Toril GIS pathways finding — 2026-09-29

The pinned Toril GIS pathway SVG contains exactly **329** features: **324 Primary Road** features and **5 Underwater Tunnel** features. Most pathways are unnamed or UUID-labelled, but the geometry is still topologically useful. Within the MVP Sword Coast clip (`FRIA bbox [-82,20,-64,55]`), 42 pathway features intersect the region.

Important validation: published primary-road vertices terminate exactly at [[Luskan]], [[Neverwinter]] and [[Waterdeep]]. Using the Toril ellipsoid and the published polylines yields approximately 233 km of road between Luskan and Neverwinter and 529 km between Neverwinter and Waterdeep. These are route-geometry distances, not direct geodesic distances.

Research implication: road names should be treated as optional semantic enrichment on top of a useful geometric network.

## DuckDB / Parquet implementation notes — 29 September 2026

Current DuckDB documentation supports Hive-style partitioned reads/writes and automatic filter pushdown on partition columns. DuckDB warns that excessive partition counts create many small files; its current partitioned-write guidance recommends aiming for roughly 100 MB or more per partition once data volume is large enough. The WDEX archive therefore uses coarse `world_id / branch_id / sim_year` partitions rather than market × commodity folders.

Sources:
- https://duckdb.org/docs/current/data/partitioning/hive_partitioning
- https://duckdb.org/docs/lts/data/partitioning/partitioned_writes
- https://duckdb.org/docs/current/guides/performance/file_formats

DuckDB-Wasm remains suitable for bounded client-side historical exploration. Current deployment documentation describes MVP, exception-handling and cross-origin-isolated/threaded worker bundles. The first WDEX browser implementation should start with a Web Worker and bounded archive samples before considering COI/threaded deployment complexity.

Source:
- https://duckdb.org/docs/stable/clients/wasm/deploying_duckdb_wasm
