# Product Roadmap

## North Star

A live, inspectable economic simulation in which physical production and trade create local prices, merchants react to incentives, commodities visibly move through the world, and D&D campaign events create traceable economic consequences.

## Milestone 0 — Economic Specification

**Goal:** remove ambiguity before coding the engine.

Deliverables:

- common units for currency, quantity, mass, volume, time and distance;
- market and inventory rules;
- commodity metadata model;
- production/consumption recipe model;
- route and shipment model;
- source provenance model;
- deterministic RNG policy;
- six-market seed dataset;
- initial 20–30 commodities.

**Exit gate:** the complete lifecycle of one grain batch from production outside [[Athkatla]] to purchase in [[Waterdeep]] can be represented without hand-waving.

## Milestone 1 — Headless Deterministic Simulation

Deliverables:

- simulation clock;
- seeded RNG;
- production;
- consumption;
- inventory;
- basic scarcity pricing;
- simple merchant arbitrage;
- route cost;
- shipments and arrivals;
- Parquet/CSV output for inspection.

**Exit gate:** same seed + same inputs produce identical output. Supply shocks and route closures create directionally sensible results.

## Milestone 2 — Durable World State

Deliverables:

- PostgreSQL schema;
- PostGIS geography;
- event log;
- world snapshots;
- load/restart/replay;
- API layer;
- WebSocket state deltas.

**Exit gate:** stop the server, restart from snapshot, replay events and reach the same world state.

## Milestone 3 — WDEX MVP

Deliverables:

- [[WDEX - Waterdeep Exchange]] dashboard;
- commodity list;
- current bid/ask/market price;
- inventory and volume;
- OHLCV chart;
- market and limit orders;
- player wallet;
- warehouse positions;
- physical cargo purchase.

**Exit gate:** buy a commodity, advance world time, observe changing fundamentals and sell later.

## Milestone 4 — Multi-Market Economy

Add [[Baldur's Gate]], [[Athkatla]], [[Neverwinter]], [[Luskan]], [[Calimport]].

Deliverables:

- local exchange/market state;
- transport cost differences;
- merchant capital allocation;
- arbitrage shipments;
- finite market depth;
- slippage;
- merchant competition.

**Exit gate:** a shortage in one city autonomously attracts shipments and gradually compresses the spread.

## Milestone 5 — Geographic Trade Map

Deliverables:

- custom Toril/Sword Coast map integration;
- settlements and production nodes;
- roads, rivers and sea lanes;
- animated shipments;
- flow thickness by volume/value;
- commodity filter;
- risk and capacity overlays.

**Exit gate:** selecting [[Timber]] visually reveals its production, storage, transport and consumption network.

## Milestone 6 — Time-Series & Analytics

Deliverables:

- Timescale/partitioned time-series layer;
- OHLCV aggregates;
- Parquet archival;
- DuckDB historical analysis;
- DuckDB-Wasm browser analytics prototype;
- market comparison charts.

**Exit gate:** query and chart a full year of six-market history without loading every raw tick into the application server.

## Milestone 7 — Production Chains

Deliverables:

- raw inputs;
- processors;
- recipes;
- workshops;
- shipyards;
- military consumption;
- substitution rules.

**Exit gate:** a [[Mirabar]] iron disruption propagates into ingots, tools, weapons, construction and eventually shipping costs.

## Milestone 8 — Advanced Merchant Ecosystem

Deliverables:

- [[OTC]] strategy;
- [[DTC/DWTC (Deepwater Trading Company)]] strategy;
- independent merchant strategy;
- warehouse behaviour;
- route preferences;
- information quality;
- risk tolerance;
- market share;
- solvency/capital.

**Exit gate:** factions produce measurably different trade patterns from the same market data.

## Milestone 9 — DM Control Room

Deliverables:

- world clock controls;
- event injection;
- route closure/opening;
- production modifiers;
- wars/blockades;
- shortage alerts;
- largest price moves;
- causal explanation view;
- save/fork timeline.

**Exit gate:** create a pirate blockade and observe the resulting economic chain without manually editing prices.

## Milestone 10 — Scenario Lab

Deliverables:

- branches from snapshots;
- multiple deterministic futures;
- Monte Carlo runs with controlled random seeds;
- scenario comparison through DuckDB;
- impact distributions.

**Exit gate:** compare a canonical timeline against a 90-day [[Luskan]] blockade without altering campaign state.

## Milestone 11 — Sword Coast Expansion

Expand to secondary ports, inland cities, mines, forests, farms, fisheries and political regions.

**Exit gate:** the six-market MVP no longer dominates all economic behaviour because regional supply networks exist around it.

## Milestone 12 — Toril Expansion

Expand toward [[Chult]], [[Moonshae Isles]], [[Whalebones]], [[Evermeet]], [[Maztica]], [[Anchorome]], [[Zakhara]] and [[Kara-Tur]].

**Exit gate:** long-distance routes produce distinct commodity corridors and make [[DTC/DWTC (Deepwater Trading Company)]] materially different from [[OTC]].
