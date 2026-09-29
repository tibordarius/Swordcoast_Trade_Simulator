# Sword Coast Economic Simulator

**Working names:** [[WDEX]], [[Sword Coast Trade Simulator]], [[Faerûn Economic Simulator]]

## Purpose

Build a live economic simulation of the [[Sword Coast]] that models settlements, production, consumption, inventories, merchant behaviour, logistics, commodity prices, trade routes, world events, and physical cargo movement. The simulation should be usable both as a D&D campaign-world engine and as an interactive trading application with exchange-style interfaces such as the [[WDEX - Waterdeep Exchange]].

The long-term ambition is a geographic and economic digital twin of the campaign version of [[Toril]], where commodity flows can be inspected on a map, traded in local markets, traced back to their origins, and altered by player actions.

## Canonical Project Files

- [[01_ARCHITECTURE]] — technical architecture and technology choices.
- [[02_ROADMAP]] — product phases, milestones, and exit gates.
- [[03_SPRINTS]] — dated delivery plan using two-week sprints.
- [[04_BACKLOG]] — epics, PBIs, user stories, engineering cards, acceptance criteria.
- [[05_RESEARCH]] — sources, lore research, economic assumptions, technical research.
- [[06_IDEAS]] — uncommitted ideas and future experiments.
- [[07_DECISIONS]] — architecture/product decision record.
- [[08_LOG]] — chronological project log.
- [[09_DATA_MODEL]] — core entities and relationships.
- [[10_SIM_ENGINE]] — simulation mechanics and deterministic tick model.
- [[11_TESTING]] — testing, validation, performance and balancing strategy.

## Source-of-Truth Rules

1. `07_DECISIONS.md` wins when an architectural choice conflicts with an older note.
2. `09_DATA_MODEL.md` and `10_SIM_ENGINE.md` define the current model, not brainstorming notes.
3. `04_BACKLOG.md` contains work that is accepted for delivery. `06_IDEAS.md` contains possibilities that are not commitments.
4. Forgotten Realms source material should be tagged as **canon**, **campaign-canon**, **inference**, or **simulation-generated**.
5. Prices should emerge from production, inventory, demand, logistics and events. Randomness may create shocks, but should not directly generate arbitrary prices.
6. The economic engine is the product. The map and exchange UI are interfaces into the engine.

## Current Delivery Assumptions

- Initial development model: one primary builder, with AI-assisted coding/research.
- Sprint length: 2 weeks.
- Baseline effort assumption: roughly 8–12 focused development hours per week. If more time is available, the sprint sequence can compress without changing dependencies.
- MVP geography: [[Luskan]], [[Neverwinter]], [[Waterdeep]], [[Baldur's Gate]], [[Athkatla]], [[Calimport]].
- MVP commodity count: 20–30.
- MVP merchant actors: [[OTC]], [[DTC/DWTC (Deepwater Trading Company)]], plus independent traders.
- First milestone: a headless deterministic economy that produces believable trade flows before any polished map work.

## Definition of MVP

MVP is reached when the following all work together:

- settlements produce and consume commodities;
- inventories accumulate and deplete;
- local prices emerge from supply/demand and market depth;
- merchant agents identify profitable inter-market opportunities;
- shipments physically take time to move between markets;
- arrivals and losses alter inventories and prices;
- users can trade in at least the [[WDEX - Waterdeep Exchange]];
- a map visualises settlements, routes and active flows;
- the world can be paused, advanced, saved and replayed deterministically;
- one DM-created event can propagate through the economy without directly editing prices.

## Immediate Next Action

Complete Sprint 0: freeze units, time model, commodity representation, source provenance rules, world seed behaviour, and the first six-market seed dataset.

## Current build checkpoint — 2026-09-29

The independent reference implementation is green through [[Sprint 20 — Hardening]], and [[Sprint 21 — MVP Release Preparation]] provides strict preflight/release gates. See [[40_MVP_CANDIDATE]] for native release blockers. The project is deliberately not labelled production-ready until Rust, PostGIS, DuckDB/Parquet and full API-to-frontend integration run in a native CI/deployment environment.
