# Sprint Plan

## Planning Assumptions

- Sprint length: 2 weeks.
- Baseline: approximately 8–12 focused hours/week.
- Start date: **29 September 2026**.
- Dates are planning anchors, not promises. Scope moves between sprints; dependencies should not.


## Current Execution Status — 29 September 2026

Work has been pulled forward as a reference-model spike to validate dependencies before waiting for calendar sprint dates.

- Sprint 0: **completed baseline** — units, provenance, six-market seed world.
- Sprint 1: deterministic oracle and Rust kernel source authored; native Rust compile still awaits an external Rust runner.
- Sprint 2: production/consumption/inventory conservation passes.
- Sprint 3: causal pricing, bid/ask and slippage passes.
- Sprint 4: shipment lifecycle, ETA, capacity and loss semantics pass.
- Sprint 5: merchant arbitrage and endogenous spread convergence pass.
- Sprint 6: 120-day integration validation passes; logistics throughput emerges as a real bottleneck.
- Sprint 6.5: GitHub Actions/native CI definition prepared, but repository creation is not exposed by the connected GitHub connector.
- Sprint 7: PostgreSQL/PostGIS schema, append-only event log and snapshot/replay semantics pass at reference level.
- Sprint 8: snapshot + monotonic delta protocol passes, including JavaScript-safe 64-bit transport.
- Sprint 9: dependency-free WDEX shell and exact market arithmetic tests pass.
- Sprint 10: wallet, warehouse, limit orders and physical cargo transfer tests pass.
- Sprint 11: six-exchange risk/freight/capacity-adjusted arbitrage network passes.
- Sprint 12: campaign-map coordinate space recovered from the Obsidian vault and local city-calibration tool passes.

All executable reference checks through Sprint 12 now run from one consolidated regression command. Native Rust/PostgreSQL runtime verification remains a hard gate, not something being silently treated as complete.

## Timeline

| Sprint | Dates | Theme | Primary Outcome |
|---|---|---|---|
| 0 | 29 Sep–11 Oct 2026 | Specification | Frozen economic units, model and seed world |
| 1 | 12–25 Oct | Simulation skeleton | deterministic clock + seeded engine |
| 2 | 26 Oct–8 Nov | Supply & inventory | production, consumption, stock |
| 3 | 9–22 Nov | Pricing & markets | scarcity pricing + market depth |
| 4 | 23 Nov–6 Dec | Logistics | routes, shipments, arrivals |
| 5 | 7–20 Dec | Merchant AI v1 | autonomous arbitrage |
| 6 | 21 Dec–3 Jan 2027 | Validation buffer | tests, balance, profiling, documentation |
| 7 | 4–17 Jan | Persistence | PostgreSQL/PostGIS schema + snapshots |
| 8 | 18–31 Jan | API & live state | Axum API + WebSocket deltas |
| 9 | 1–14 Feb | WDEX shell | exchange UI and market overview |
| 10 | 15–28 Feb | Trading | orders, wallet, warehouse, physical cargo |
| 11 | 1–14 Mar | Multi-market | six exchanges + slippage + trade spreads |
| 12 | 15–28 Mar | Map foundation | MapLibre, geography, PMTiles/data pipeline |
| 13 | 29 Mar–11 Apr | Live logistics map | deck.gl flows, ships, risk overlays |
| 14 | 12–25 Apr | Historical markets | Timescale/partitions, OHLCV, aggregates |
| 15 | 26 Apr–9 May | DuckDB analytics | Parquet archive + historical explorer |
| 16 | 10–23 May | Production chains | recipes, transformation networks |
| 17 | 24 May–6 Jun | Advanced factions | OTC/DTC/independent behaviour profiles |
| 18 | 7–20 Jun | DM control room | events, controls, causal explanations |
| 19 | 21 Jun–4 Jul | Scenario branches | fork/replay/counterfactual timelines |
| 20 | 5–18 Jul | Hardening | benchmarks, determinism, recovery, UX polish |
| 21 | 19 Jul–1 Aug | MVP release | documented playable campaign build |

## Sprint 0 — Specification

### Goals
- freeze the first economic ontology;
- define all units;
- build six-market seed data;
- create the initial backlog and architecture tests.

### Cards
- SPEC-001 Define currency smallest unit.
- SPEC-002 Define commodity quantity/unit conventions.
- SPEC-003 Define time/tick conventions.
- SPEC-004 Define inventory/reserve concepts.
- SPEC-005 Define market price state.
- SPEC-006 Define source provenance classes.
- DATA-001 Create six-market settlement seed.
- DATA-002 Create initial commodity catalogue.
- DATA-003 Create first route matrix.

### Sprint exit
A hand-worked example can simulate one grain shipment end-to-end using only documented rules.

## Sprint 1 — Simulation Skeleton

### Cards
- ENG-001 Create Rust workspace.
- ENG-002 Implement simulation clock.
- ENG-003 Implement stable IDs.
- ENG-004 Implement seeded RNG streams.
- ENG-005 Implement phase scheduler.
- ENG-006 Implement deterministic delta commit.
- TEST-001 Determinism golden test.
- OBS-001 Basic tick profiling output.

### Sprint exit
10,000 ticks can execute twice with identical hashes.

## Sprint 2 — Supply, Consumption & Inventory

### Cards
- SIM-001 Production sites.
- SIM-002 Consumption sectors.
- SIM-003 Market inventory ledger.
- SIM-004 Reserve targets.
- SIM-005 spoilage/perishability hooks.
- TEST-002 conservation-of-goods tests.

## Sprint 3 — Pricing & Market Depth

### Cards
- MKT-001 Base price metadata.
- MKT-002 scarcity function.
- MKT-003 demand pressure.
- MKT-004 bid/ask spread.
- MKT-005 liquidity curve.
- MKT-006 slippage.
- MKT-007 price explanation components.

## Sprint 4 — Logistics

### Cards
- LOGI-001 Route graph.
- LOGI-002 route capacity.
- LOGI-003 transport costs.
- LOGI-004 shipment lifecycle.
- LOGI-005 ETA calculation.
- LOGI-006 route-risk loss event.
- LOGI-007 derived map position.

## Sprint 5 — Merchant AI v1

### Cards
- AI-001 merchant capital.
- AI-002 market observation.
- AI-003 opportunity scoring.
- AI-004 risk-adjusted ROI.
- AI-005 shipment purchase decision.
- AI-006 profit/loss accounting.
- TEST-003 arbitrage convergence test.

## Sprint 6 — Validation Buffer

Purposefully keep this sprint feature-light.

- profile engine;
- tune prices;
- test shock propagation;
- remove accidental positive-feedback explosions;
- document benchmark world;
- create replay fixtures.

## Sprints 7–21

Detailed PBIs for the remaining sprints are maintained in [[04_BACKLOG]]. Sprint scope should be pulled from Ready items based on dependency completion rather than blindly following the calendar.

## Post-MVP

After Sprint 21:

- secondary Sword Coast settlements;
- production chain depth;
- ship classes and chartering;
- weather and seasons;
- piracy ecology;
- political/tariff systems;
- futures/forward delivery contracts;
- browser-side Rust/WASM scenarios;
- PGlite offline mode;
- Toril expansion.


## Execution checkpoint — 29 September 2026

Reference implementation status has advanced ahead of the calendar plan:

- Sprint 0–20: reference-complete and covered by the consolidated regression suite;
- Sprint 15 native DuckDB/Parquet execution is prepared as a CI gate but cannot run in the current sandbox;
- native Rust/PostgreSQL/PostGIS execution remains gated by tool availability/CI environment rather than design work;
- Sprint 21 release-preparation harness is reference-complete; the reference release gate passes while the strict native gate remains blocked by missing native toolchains/services in this runtime.
- The project remains an [[WDEX MVP Candidate]], not a falsely declared production release.
