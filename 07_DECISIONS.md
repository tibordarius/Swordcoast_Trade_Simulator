# Architecture & Product Decision Record

## ADR-001 — Simulation first, visualisation second

**Status:** Accepted  
**Date:** 2026-09-29

The project will prove a believable headless economy before investing heavily in map animation or exchange polish.

**Reason:** a beautiful trading interface over arbitrary/random prices would fail the core product idea.

---

## ADR-002 — Modular monolith for MVP

**Status:** Accepted  
**Date:** 2026-09-29

Start with one application/API and one authoritative simulation worker per world.

**Rejected for MVP:** microservices, Kafka, Kubernetes.

**Reason:** deterministic ordering and development speed matter more than horizontal service decomposition.

---

## ADR-003 — Rust simulation core

**Status:** Proposed/Preferred  
**Date:** 2026-09-29

Use Rust for the simulation engine and likely Axum for the server API.

**Reason:** deterministic performance, memory efficiency, parallelism and a future path to Rust/WASM.

**Validation required:** build a small benchmark spike before locking in.

---

## ADR-004 — PostgreSQL is durable world state, not the simulation loop

**Status:** Accepted  
**Date:** 2026-09-29

The hot numerical simulation executes in memory. PostgreSQL stores authoritative durable state, configuration and checkpoints.

---

## ADR-005 — PostGIS for geography

**Status:** Accepted  
**Date:** 2026-09-29

Settlements, regions, production zones and route geometry live in a spatial model compatible with PostGIS.

---

## ADR-006 — DuckDB + Parquet for historical analytics

**Status:** Accepted direction  
**Date:** 2026-09-29

Recent data remains operational/time-series data. Long historical data moves to Parquet and is queried with DuckDB.

DuckDB-Wasm will be benchmarked for browser-side historical exploration.

---

## ADR-007 — Prices are emergent

**Status:** Accepted  
**Date:** 2026-09-29

Randomness may alter harvests, risks, weather, production and events. It must not directly assign arbitrary market prices.

Prices derive from economic state and market mechanics.

---

## ADR-008 — One authoritative writer per world

**Status:** Accepted  
**Date:** 2026-09-29

Parallel calculation is allowed, but one ordered commit controls authoritative world transitions.

---

## ADR-009 — Fixed-point values for authoritative quantities

**Status:** Preferred  
**Date:** 2026-09-29

Currency and core commodity quantities should use integer/fixed-point representation where feasible.

---

## ADR-010 — Provenance is part of the data model

**Status:** Accepted  
**Date:** 2026-09-29

Data must distinguish Forgotten Realms canon, Whale Campaign canon, inference and simulation-generated state.

---

## ADR-011 — Do not simulate individual civilians

**Status:** Accepted  
**Date:** 2026-09-29

Population demand is aggregated into sectors. Significant merchant organizations may be simulated individually because their behaviour matters to gameplay.

---

## ADR-012 — Ship positions are derived

**Status:** Accepted  
**Date:** 2026-09-29

Persist route, departure, ETA and state. Derive current map position client-side or server-side when queried. Do not write continuous position ticks to the database.

---

## ADR-013 — Five-minute authoritative base tick

**Status:** Accepted  
**Date:** 2026-09-29

The authoritative simulation clock advances in five-minute ticks. Subsystems are multi-rate/event-driven and do not execute every base tick.

---

## ADR-014 — Commodity-specific base units

**Status:** Accepted  
**Date:** 2026-09-29

There is no universal trade unit. Each commodity has a semantic unit such as kg, litre, cubic metre, bolt or item-equivalent, represented using integer fixed-point sub-units. Cargo mass and volume are modeled separately.

---

## ADR-015 — Deterministic RNG algorithm begins dependency-free

**Status:** Accepted for kernel v0  
**Date:** 2026-09-29

Use a fully specified SplitMix64 implementation for the first deterministic kernel and derive named subsystem streams from stable identifiers. This avoids relying on external RNG crate implementation stability for replay compatibility.

---

## ADR-016 — AI remains outside the deterministic boundary

**Status:** Accepted  
**Date:** 2026-09-29

LLMs may translate DM prose into proposed structured events, extract lore or explain deterministic outputs. Only approved structured events enter the authoritative event log. LLM calls never execute inside the simulation tick loop.

---

## ADR-017 — Monetary precision is milli-copper, superseding cp-only storage

**Status:** Accepted  
**Date:** 2026-09-29

Sprint 3 demonstrated that wholesale unit quotations require sub-copper precision (`1.75 cp/kg` is a normal case). Authoritative monetary values and quotes therefore use integer **milli-copper pieces (mcp)** where `1 cp = 1,000 mcp`.

This supersedes the cp-only internal precision stated in the first Sprint 0 draft. No floating-point authoritative prices are allowed.

---

## ADR-018 — Pricing v0 is causal and piecewise-integer

**Status:** Accepted as calibration baseline  
**Date:** 2026-09-29

The first pricing function uses integer basis-point components for reserve pressure, unmet demand, effective incoming supply and risk. The function is intentionally simple, monotonic and explainable. Its elasticity parameters are calibration data, not permanent economic truth.

---

## ADR-019 — Toril GIS is canonical world geography

**Status:** Accepted  
**Date:** 2026-09-29

Use the Geospatial Grimoire Toril GIS coordinate framework as the canonical geography substrate. Store source geography in Toril GCS using the FRIA prime meridian so upstream GeoJSON can be consumed without coordinate rewriting. Display lore-facing longitude relative to the Myth Drannor prime meridian.

The original 6600×10200 Sword Coast campaign raster remains an optional georeferenced overlay and physical-table reference, not the authoritative world coordinate system.

---

## ADR-020 — Geography inputs are version-pinned

**Status:** Accepted  
**Date:** 2026-09-29

External GIS datasets are pinned by repository commit and export batch. Geography upgrades are explicit migrations. Never silently ingest an upstream `main` snapshot into an existing campaign world.

---

## ADR-021 — Toril distance uses Toril ellipsoid math

**Status:** Accepted  
**Date:** 2026-09-29

MapLibre may render Toril degrees numerically, but WGS84/Earth distance calculations are forbidden for simulation. Distance/area calculations use Toril's ellipsoid (`a=6410 km`, `b=6370 km`) or precomputed integer measurements derived from it.


## ADR-022 — Published Toril GIS pathways are authoritative overland geometry where available

**Status:** Accepted  
**Date:** 2026-09-29

For overland freight routing, use version-pinned `srf_civ_pathways_ln` geometry from Toril GIS when available. Build routing edges from the actual polyline vertices rather than reducing a published road to a straight settlement-to-settlement link. Missing road names do not invalidate usable published geometry; retain source UUIDs and provenance.

---

## ADR-023 — Sea lanes remain a separate navigation graph

**Status:** Accepted  
**Date:** 2026-09-29

Published road geometry must not be repurposed as maritime routing. Sea lanes are a separate graph informed by ports, coastlines, hazards, prevailing conditions, campaign routes and navigational knowledge. Straight geodesics remain clearly marked placeholders until a navigable sea graph replaces them.


---

## ADR-024 — Market history uses hot/warm/cold tiers

**Status:** Accepted  
**Date:** 2026-09-29

Recent authoritative observations live in PostgreSQL (`market_tick`). Reusable OHLCV summaries live in `market_candle` or an equivalent Timescale continuous aggregate. Old raw history is archived to partitioned Parquet and queried with DuckDB rather than kept indefinitely as ordinary transactional rows.

Core migrations remain valid on ordinary PostgreSQL/PostGIS. Timescale is an optional deployment accelerator, not a hard dependency of the authoritative simulation schema.


## ADR-025 — Historical Parquet uses coarse Hive partitions

**Status:** Accepted  
**Date:** 2026-09-29

Partition cold market history by `world_id / branch_id / sim_year`, then sort within files by market, commodity and tick. Do not create a partition directory per market × commodity combination. The design follows DuckDB guidance to avoid excessive small partitions and relies on Parquet row-group/filter pushdown for finer pruning.

---

## ADR-026 — Cold history is an immutable analytical projection

**Status:** Accepted  
**Date:** 2026-09-29

PostgreSQL/snapshots/event logs remain authoritative. Parquet history may be regenerated from an authoritative checkpoint but never becomes a source of simulation truth. Archive batches carry row counts, hashes and source-version metadata.

---

## ADR-027 — Route ROI remains mark-to-market until cargo disposal is attributable

**Status:** Accepted  
**Date:** 2026-09-29

Until a cargo lot can be linked to one or more destination sale executions, analytical route profitability is labelled **arrival mark-to-market ROI**. Do not describe it as realized profit.

---

## ADR-028 — Production transformations must be explicit recipes

**Status:** Accepted  
**Date:** 2026-09-29

Commodity creation/destruction outside source production or consumption must occur through a recorded recipe with explicit inputs and outputs. This gives downstream price shocks a traceable physical cause. Recipe ratios are calibration data unless separately sourced as campaign/FR canon.

---

## ADR-029 — Faction trading behavior is profile-driven

**Status:** Accepted  
**Date:** 2026-09-29

The merchant engine must not branch on faction names. [[OTC]], [[DTC/DWTC (Deepwater Trading Company)]] and future factions bind to generic strategy profiles with data-driven risk, information, distance, bulk/value-density and warehousing preferences.

---

## ADR-030 — DM events modify causes, never authoritative prices

**Status:** Accepted  
**Date:** 2026-09-29

DM controls may alter production, consumption, routes, risks, costs, taxes, port throughput and information. Direct price overrides are rejected by the event contract so market prices remain consequences of world state.

---

## ADR-031 — Scenario branches are snapshot descendants

**Status:** Accepted  
**Date:** 2026-09-29

Counterfactuals reference a parent snapshot and maintain independent deterministic seed namespaces and event logs. Branch state is never allowed to mutate the canonical parent.

---

## ADR-032 — The repository root is the full project pack

**Status:** Accepted  
**Date:** 2026-09-29

When a Git repository is created, the repository root contains docs, `seed/`, `tools/`, `reference/` and `code/`. Rust commands use `code/` as their working directory. This keeps CI able to run both native code and independent reference oracles.
