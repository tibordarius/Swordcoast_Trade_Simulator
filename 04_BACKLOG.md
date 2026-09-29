# Product Backlog

## Backlog Conventions

**Priority:** P0 = required for current milestone, P1 = important, P2 = later.

**Story points:** Fibonacci-like 1, 2, 3, 5, 8, 13. Points are relative complexity, not hours.

**PBI format:**

- ID
- User story
- Acceptance criteria
- Dependencies
- Priority
- Points

---

# Epic E01 — World Model

## PBI WM-001 — Settlement registry

**User story:** As the simulation, I need every settlement to have a stable identity and economic metadata so that production, demand, markets and routes can reference the same place reliably.

**Acceptance criteria:**
- settlement has stable ID, name, type, region and coordinates;
- optional port/river/road capabilities are represented;
- source provenance is attached;
- IDs remain stable across imports.

**Dependencies:** none  
**Priority:** P0  
**Points:** 3

## PBI WM-002 — Commodity catalogue

**User story:** As a DM/data maintainer, I want commodities described consistently so that all markets use the same units and rules.

**Acceptance criteria:**
- commodity has ID, name, family, base unit, density/space class, perishability and reference price;
- legal/contraband flags are possible;
- source provenance is recorded;
- 20–30 MVP commodities exist.

**Dependencies:** WM-001  
**Priority:** P0  
**Points:** 5

## PBI WM-003 — Canon/inference provenance

**User story:** As a lore researcher, I want every economic fact to declare whether it is Forgotten Realms canon, campaign canon, inference, or simulation-generated so that the world model remains auditable.

**Acceptance criteria:**
- source type enum exists;
- source title/page/URL can be stored;
- inferred records can explain derivation;
- UI/API can surface provenance later.

**Priority:** P0  
**Points:** 3

---

# Epic E02 — Simulation Kernel

## PBI SIM-001 — Deterministic simulation clock

**User story:** As a developer, I want one authoritative tick clock so that the complete economy advances in a reproducible order.

**Acceptance criteria:**
- world has tick and in-world datetime;
- speed multiplier does not alter results;
- pause/resume works;
- same seed/input stream produces same state hash.

**Priority:** P0  
**Points:** 5

## PBI SIM-002 — Seeded RNG streams

**User story:** As a developer, I want deterministic random streams per subsystem so that random world events can be replayed and debugged.

**Acceptance criteria:**
- no unseeded RNG in engine;
- subsystem stream creation is documented;
- adding a UI request does not perturb economic RNG;
- replay test passes.

**Priority:** P0  
**Points:** 5

## PBI SIM-003 — Multi-rate scheduler

**User story:** As the engine, I want subsystems to run only at meaningful frequencies so that performance scales with world activity rather than visual frame rate.

**Acceptance criteria:**
- market, production, consumption, merchant strategy and slow demographics can have separate cadence;
- event-driven tasks can schedule future callbacks/events;
- idle systems consume negligible work.

**Priority:** P0  
**Points:** 8

---

# Epic E03 — Production & Consumption

## PBI ECON-001 — Production sites

**User story:** As the world simulation, I need farms, mines, fisheries and workshops to produce goods so that commodities originate somewhere real.

**Acceptance criteria:**
- production rate can vary by season/modifier;
- production consumes optional inputs;
- output is added to an inventory;
- shortages of required inputs constrain output.

**Priority:** P0  
**Points:** 5

## PBI ECON-002 — Consumption sectors

**User story:** As the world simulation, I need populations and industries to consume goods so that demand is grounded in economic activity.

**Acceptance criteria:**
- consumption is represented by aggregate sectors;
- demand can be essential, normal or discretionary;
- unmet essential demand is recorded;
- consumption varies by population/event modifiers.

**Priority:** P0  
**Points:** 5

## PBI ECON-003 — Production recipes

**User story:** As a worldbuilder, I want commodities to transform into other commodities so that upstream shortages can propagate through industry.

**Acceptance criteria:**
- recipe supports multiple inputs/outputs;
- facility capacity limits throughput;
- missing inputs reduce or stop output;
- provenance can trace transformed batches.

**Priority:** P1  
**Points:** 8

---

# Epic E04 — Markets & Pricing

## PBI MKT-001 — Inventory-driven price

**User story:** As a trader, I want prices to reflect local scarcity so that market movements have understandable causes.

**Acceptance criteria:**
- price responds monotonically to stock/reserve ratio within configured bounds;
- demand and risk modifiers are composable;
- price does not become negative or explode to infinity;
- each change exposes contribution components.

**Priority:** P0  
**Points:** 8

## PBI MKT-002 — Market depth and slippage

**User story:** As a trader, I want large orders to move the market so that limited local liquidity matters.

**Acceptance criteria:**
- available liquidity exists across price levels or an equivalent mathematical curve;
- large orders receive worse average execution;
- inventory is reserved/removed correctly;
- execution is deterministic.

**Priority:** P0  
**Points:** 8

## PBI MKT-003 — OHLCV generation

**User story:** As a WDEX user, I want candles and volume so that I can read the market like a trading platform.

**Acceptance criteria:**
- interval aggregation produces open/high/low/close/volume;
- empty intervals are handled consistently;
- longer intervals can derive from smaller intervals or continuous aggregates.

**Priority:** P0  
**Points:** 5

## PBI MKT-004 — Explain price movement

**User story:** As a DM/trader, I want to understand why a price moved so that market behaviour can become campaign information.

**Acceptance criteria:**
- current price stores/calculates major drivers;
- explanation separates inventory, consumption, incoming supply, risk and speculation;
- explanation can link to source events or shipments.

**Priority:** P1  
**Points:** 8

---

# Epic E05 — Logistics

## PBI LOG-001 — Route graph

**User story:** As a merchant, I want routes with distance, cost, risk, capacity and travel time so that moving goods has consequences.

**Acceptance criteria:**
- road/river/sea route types supported;
- route edges can be closed or modified;
- path cost can combine time, money and expected risk;
- route geometry is map-renderable.

**Priority:** P0  
**Points:** 8

## PBI LOG-002 — Shipment lifecycle

**User story:** As the simulation, I want cargo shipments to physically take time so that markets do not instantly equilibrate.

**Acceptance criteria:**
- shipment records origin, destination, commodity, quantity, owner, departure and ETA;
- inventory leaves origin and arrives at destination exactly once;
- loss/diversion events are supported;
- map position is derived from route progress.

**Priority:** P0  
**Points:** 8

## PBI LOG-003 — Route cache

**User story:** As the engine, I want frequently used route costs cached so that merchant opportunity scans do not repeatedly solve the same graph.

**Acceptance criteria:**
- cache invalidates on material route changes;
- multiple optimization profiles may coexist;
- benchmark demonstrates material reduction in routing work.

**Priority:** P1  
**Points:** 5

---

# Epic E06 — Merchant Agents

## PBI AI-001 — Arbitrage merchant

**User story:** As the world, I want independent merchants to move goods toward profitable shortages so that the economy self-corrects.

**Acceptance criteria:**
- merchant observes markets according to information access;
- calculates purchase, transport, tax and expected-loss cost;
- respects capital and cargo capacity;
- selects positive risk-adjusted opportunities;
- records realized P&L.

**Priority:** P0  
**Points:** 13

## PBI AI-002 — OTC strategy profile

**User story:** As a DM, I want [[OTC]] to behave like a vertically integrated aggressive trading company so that faction identity emerges economically.

**Acceptance criteria:**
- favours bulk/high-volume lanes;
- can warehouse inventory;
- can prefer controlled routes;
- strategy parameters are data-driven, not hardcoded names.

**Priority:** P1  
**Points:** 8

## PBI AI-003 — DTC strategy profile

**User story:** As a DM, I want [[DTC/DWTC (Deepwater Trading Company)]] to favour sparse long-distance opportunities so that its network feels distinct from OTC.

**Acceptance criteria:**
- higher tolerance for remote routes;
- stronger distant-market information;
- preference for higher-value lower-frequency cargo;
- behaviour measurable against OTC benchmark.

**Priority:** P1  
**Points:** 8

---

# Epic E07 — Persistence & Replay

## PBI DATA-001 — PostgreSQL operational schema

**User story:** As the application, I need durable current state so that campaign worlds survive restarts.

**Acceptance criteria:**
- normalized core entities;
- migrations automated;
- foreign keys/indexes defined;
- bulk state checkpoint path exists.

**Priority:** P0  
**Points:** 8

## PBI DATA-002 — World snapshots

**User story:** As a DM, I want periodic snapshots so that the world can be restored, forked and replayed efficiently.

**Acceptance criteria:**
- snapshot includes all authoritative simulation state;
- snapshot checksum stored;
- restore test reproduces state;
- snapshot interval configurable.

**Priority:** P0  
**Points:** 8

## PBI DATA-003 — Append-only input event log

**User story:** As a developer, I want commands/world interventions recorded so that simulation history is auditable and replayable.

**Acceptance criteria:**
- DM/player/system input events receive sequence number;
- immutable after commit;
- replay from snapshot + events works;
- schema version recorded.

**Priority:** P0  
**Points:** 8

---

# Epic E08 — WDEX Trading UI

## PBI UI-001 — WDEX market overview

**User story:** As a trader, I want a Waterdeep exchange screen showing prices, change, volume and trend so that I can quickly understand the market.

**Acceptance criteria:**
- sortable commodity table;
- live delta updates;
- daily/tenday change;
- inventory/volume indicators;
- commodity detail navigation.

**Priority:** P0  
**Points:** 8

## PBI UI-002 — Commodity detail

**User story:** As a trader, I want a commodity detail page with candles, volume, stock, imports, exports and news so that I can evaluate a trade.

**Acceptance criteria:**
- OHLCV chart;
- interval selector;
- current inventory;
- incoming/outgoing shipments;
- major causal events.

**Priority:** P0  
**Points:** 8

## PBI UI-003 — Market and limit orders

**User story:** As a trader, I want market and limit orders so that I can trade without always executing immediately.

**Acceptance criteria:**
- market buy/sell;
- limit buy/sell;
- insufficient capital/inventory rejected;
- execution generates immutable trade record;
- slippage shown before/after execution.

**Priority:** P0  
**Points:** 13

## PBI UI-004 — Warehouse vs physical cargo

**User story:** As a trader/captain, I want to distinguish warehouse ownership from cargo loaded on my ship so that financial positions and logistics interact.

**Acceptance criteria:**
- warehouse balance separate from vessel cargo;
- transfer action exists;
- capacity enforced;
- physical cargo can be shipped to another market.

**Priority:** P0  
**Points:** 8

---

# Epic E09 — Geographic Visualization

## PBI MAP-001 — Custom map base

**User story:** As a DM, I want the campaign map in the application so that economic data appears on the geography I actually use.

**Acceptance criteria:**
- custom map loads without public-world geographic assumptions;
- settlements align with campaign coordinates;
- zoom/pan/labels work;
- static assets are cacheable.

**Priority:** P0  
**Points:** 13

## PBI MAP-002 — Commodity flow layer

**User story:** As a DM/trader, I want to select a commodity and see its active flows so that supply chains become visible.

**Acceptance criteria:**
- route thickness represents volume;
- optional colour/intensity represents value or pressure;
- routes update from live deltas;
- filtering by faction and time works.

**Priority:** P0  
**Points:** 13

## PBI MAP-003 — Animated shipments

**User story:** As a user, I want active ships/caravans to move smoothly without the server streaming positions continuously.

**Acceptance criteria:**
- client interpolates from departure/ETA/path;
- animation remains smooth with thousands of visible shipments in benchmark mode;
- server position updates are unnecessary under normal travel.

**Priority:** P1  
**Points:** 8

---

# Epic E10 — Analytics

## PBI ANA-001 — Historical Parquet archive

**User story:** As an analyst, I want old market data archived in Parquet so that history remains cheap and queryable.

**Acceptance criteria:**
- partition convention documented;
- export/compaction job exists;
- DuckDB can query archive directly;
- retention does not break charts.

**Priority:** P1  
**Points:** 8

## PBI ANA-002 — DuckDB research queries

**User story:** As a DM/researcher, I want to query historical spreads, volatility and route profitability so that I can understand the simulated economy.

**Acceptance criteria:**
- canned queries for spread, volume, volatility, market share and route ROI;
- query Postgres current state plus Parquet history where necessary;
- results reproducible.

**Priority:** P1  
**Points:** 8

## PBI ANA-003 — DuckDB-Wasm browser prototype

**User story:** As a user, I want historical exploration to execute locally where practical so that the server is not required for every analytical interaction.

**Acceptance criteria:**
- run inside Web Worker;
- reads bounded Parquet sample;
- returns Arrow/table data;
- browser memory/performance benchmark documented.

**Priority:** P2  
**Points:** 8

---

# Epic E11 — DM World Control

## PBI DM-001 — Simulation controls

**User story:** As the DM, I want pause, resume and speed controls so that economic time follows campaign time.

**Acceptance criteria:** pause, resume, ×1, accelerated mode, advance-to-date, safe save before large jump.

**Priority:** P0  
**Points:** 5

## PBI DM-002 — Event injection

**User story:** As the DM, I want to create economic events such as blockades, mine collapses and storms so that session consequences can enter the simulation cleanly.

**Acceptance criteria:**
- event has type, scope, start, duration and modifiers;
- preview affected systems;
- event recorded in immutable log;
- prices are not manually overwritten.

**Priority:** P0  
**Points:** 8

## PBI DM-003 — Why did this move?

**User story:** As the DM, I want a causal trace from a price move to underlying shortages, routes, shipments and events so that I can turn economics into story information.

**Acceptance criteria:**
- top contributing factors shown;
- contributions trace to concrete entities/events;
- can distinguish public knowledge from DM-only knowledge.

**Priority:** P1  
**Points:** 13

---

# Epic E12 — Scenarios

## PBI SCN-001 — Fork timeline

**User story:** As the DM, I want to fork the world from a snapshot so that I can test an alternative without changing campaign canon.

**Acceptance criteria:**
- child branch references parent snapshot;
- independent event log;
- branch can run at high speed;
- branch state never leaks into main.

**Priority:** P1  
**Points:** 13

## PBI SCN-002 — Monte Carlo batch

**User story:** As the DM, I want to run many controlled random futures so that I can see ranges of plausible economic outcomes from a world event.

**Acceptance criteria:**
- configurable run count and horizon;
- independent seed derivation;
- result summaries stored in analytical form;
- canonical world remains unchanged.

**Priority:** P2  
**Points:** 13

---

# Technical Chores / Enablers

- DEV-001 repository setup and CI.
- DEV-002 migration framework.
- DEV-003 structured logging.
- DEV-004 benchmark harness.
- DEV-005 state hashing.
- DEV-006 error taxonomy.
- DEV-007 schema versioning.
- DEV-008 backup/restore automation.
- DEV-009 seed-world import validation.
- DEV-010 performance regression dashboard.
