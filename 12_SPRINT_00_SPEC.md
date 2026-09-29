# Sprint 0 Economic Specification

**Status:** Working baseline accepted for implementation unless superseded in `07_DECISIONS.md`.
**Date:** 2026-09-29
**Scope:** [[Sword Coast Economic Simulator]] MVP.

## Objective

Freeze the economic vocabulary that every subsystem will share. Code should not invent its own meanings for price, quantity, stock, time, reserve, provenance, or cargo capacity.

## 1. Currency

### Internal representation

All monetary values are stored as signed 64-bit integers in **copper pieces (cp)**.

- 1 cp = 1 internal monetary unit
- 1 sp = 10 cp
- 1 ep = 50 cp
- 1 gp = 100 cp
- 1 pp = 1,000 cp

The UI may display gp/sp/cp combinations, but the engine never stores floating-point currency.

### Rationale

- exact accounting;
- deterministic simulation;
- no floating-point drift;
- local coin systems can later be implemented as exchange instruments rather than changing the accounting unit.

## 2. Commodity Quantities

There is **no universal physical commodity unit**. Each commodity declares a semantic base unit appropriate to that good.

Examples:

- grain: kilogram equivalent;
- wine: litre equivalent;
- timber: cubic metre equivalent;
- weapons: item;
- silk: bolt;
- furs: pelt;
- gems: standardized parcel/carat-equivalent later.

Internally, quantity is stored as an integer number of `1/1000 base_unit` to permit fractional trade without floating point.

Each commodity also declares:

- `base_unit_kind`;
- `mass_grams_per_base_unit` where meaningful;
- `volume_cm3_per_base_unit` where meaningful;
- `units_per_trade_lot` for UI presentation;
- `perishability_class`;
- `quality_model`;
- `fungibility_class`.

This separates **economic quantity** from **cargo space**.

## 3. Cargo Capacity

Cargo logistics use two constraints:

1. mass capacity;
2. volume capacity.

A vessel or warehouse is full when either limit is reached.

For MVP, ship classes may use simplified mass/volume profiles derived from campaign ship cargo tonnage. The engine should not assume that one ton of silk occupies the same practical hold capacity as one ton of iron.

## 4. Time

### Authoritative tick

**1 simulation tick = 5 in-world minutes.**

Therefore:

- 12 ticks = 1 hour;
- 288 ticks = 1 day;
- 2,880 ticks = 1 tenday;
- 105,120 ticks ≈ one 365-day year before special calendar handling.

### Multi-rate cadence

The engine clock advances in 5-minute ticks, but subsystems only run when due.

Initial cadence:

- order matching / market events: 1 tick or event-driven;
- price recomputation: when material state changes, with a maximum 15-minute refresh;
- production: hourly unless site-specific;
- household consumption: hourly aggregation;
- industrial consumption: hourly/event-driven;
- merchant opportunity scan: every 1–6 hours by strategy;
- route recomputation: event-driven;
- population/demographic updates: tendaily or slower;
- seasonal effects: daily/season boundary.

### Calendar

The engine stores authoritative `sim_tick` first. A [[Calendar of Harptos]] adapter converts ticks to DR calendar dates. Festival days and Shieldmeet are display/calendar concerns and must not alter tick determinism.

## 5. Inventory Vocabulary

For market `M` and commodity `C`:

- `on_hand`: physically present stock;
- `reserved`: committed but not yet removed stock;
- `available = on_hand - reserved`;
- `target_reserve`: desired buffer stock;
- `incoming_committed`: shipments contractually/physically headed to this market;
- `outgoing_committed`: stock already promised for departure;
- `unmet_demand`: demand that could not be satisfied;
- `days_of_cover`: available stock divided by recent normalized consumption.

Incoming stock must not be treated as equivalent to stock in the warehouse. Its contribution to expectations is discounted by ETA and reliability.

## 6. Price Vocabulary

Every market/commodity pair may have:

- `reference_price_cp`: long-run anchor used for initialization and sanity checks;
- `fundamental_price_cp`: current model-derived value before order-book impact;
- `best_bid_cp`;
- `best_ask_cp`;
- `last_trade_cp`;
- `mark_price_cp`: stable display/valuation price;
- `spread_cp`;
- `market_depth`;
- `volatility_state`.

A price is never changed directly by a world event. Events change production, consumption, inventories, route risk, costs, information, or orders; prices react.

## 7. Reserve Model v0

Each market/commodity pair stores a target reserve in **days of normal consumption** rather than an arbitrary quantity where possible.

Initial classes:

- essential food: 20–40 days;
- ordinary consumables: 10–25 days;
- industrial inputs: 10–30 days;
- luxury imports: 5–15 days;
- rare goods: no fixed reserve or bespoke policy.

Exact settlement values remain seed-data parameters and are not universal constants.

## 8. Provenance Classes

Every seeded economic fact must be classified as one of:

- `fr_canon` — directly supported by Forgotten Realms source material;
- `campaign_canon` — established in the Whale Campaign;
- `inference` — economic/worldbuilding inference from known facts;
- `simulation_generated` — emergent state created by the engine.

Optional source metadata:

- title;
- edition;
- page/location;
- URL/file;
- quoted/paraphrased basis;
- confidence;
- note explaining inference.

No inference silently upgrades itself to canon.

## 9. Fungibility and Batches

Not all commodities need persistent batch provenance.

### Fully fungible

Examples: ordinary grain, coal, iron ore.

Batches may merge when quality and ownership rules allow it.

### Quality-sensitive

Examples: wine, timber, silk, gems.

Batch quality matters and aggregation may lose information.

### Provenance-sensitive

Examples: magical reagents, relic material, whale products, contraband, named vintages.

Batch origin should remain traceable.

## 10. World Seed and Determinism

Each campaign world has:

- `world_seed`;
- `simulation_version`;
- `branch_id`;
- ordered external input event stream.

Random streams are derived from stable subsystem/entity identifiers. UI activity must never consume simulation randomness.

Same snapshot + same simulation version + same ordered input events = same state hash.

## 11. MVP Market Set

The first six markets are:

1. [[Waterdeep]] / WDEX;
2. [[Baldur's Gate]] / BGEX;
3. [[Athkatla]] / ATHEX;
4. [[Calimport]] / CALEX;
5. [[Neverwinter]] / NWEX;
6. [[Luskan]] / LUSEX.

These markets intentionally span northern resource production, central redistribution, southern agriculture/manufacturing, and long-distance luxury trade.

## 12. MVP Commodity Families

The initial catalogue should prove several economic behaviours rather than maximize lore coverage:

### Food staples
- grain;
- fresh fish;
- salted fish;
- livestock/meat;
- salt.

### Raw/industrial
- ordinary timber;
- ship timber;
- iron ore;
- iron ingots;
- hides;
- wool.

### Manufactured
- cloth/textiles;
- rope;
- pottery;
- tools;
- weapons;
- armor;
- ale;

### Luxury / high-value
- wine;
- spices;
- silk;
- gems;
- whale oil;
- alchemical reagents.

**Total v0: 24 commodities.**

## 13. Required Sprint 0 Validation

Before Sprint 1 begins we must be able to hand-calculate one shipment using only documented rules:

`production -> local inventory -> local market purchase -> reservation -> shipment departure -> travel -> arrival -> destination inventory -> sale -> trader P&L`

No part of that chain may require an undocumented assumption.

## Accepted Sprint 0 Decisions

- [x] integer cp currency;
- [x] per-commodity semantic base units;
- [x] integer milli-base-unit quantities;
- [x] mass + volume logistics constraints;
- [x] five-minute authoritative tick;
- [x] subsystem multi-rate scheduling;
- [x] reserve targets based primarily on days of cover;
- [x] four provenance classes;
- [x] six MVP markets;
- [x] 24-commodity v0 catalogue.

## Still Open

- exact market clearing formula;
- exact scarcity curve;
- quality representation;
- initial reference prices;
- initial production/consumption volumes;
- exact route distances/travel times;
- whether fresh fish should decay continuously or by discrete quality bands;
- whether livestock is represented as head-count or standardized livestock units.
