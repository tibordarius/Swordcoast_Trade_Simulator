# Seed World v0

**Status:** Provisional research seed. Economic magnitudes are not yet calibrated.
**Date:** 2026-09-29

## Purpose

Define the minimum economic topology needed to test the engine. This file intentionally distinguishes source-supported trade identities from numerical assumptions that still require balancing.

## Source Notes

The uploaded 3e *Forgotten Realms Campaign Setting* explicitly describes:

- [[Sword Coast North]] imports: books, manufactured items, magic items, miners, pottery, spices;
- [[Sword Coast North]] exports: gems, leather goods, mercenaries, Neverwinter's crafts, precious metals, timber;
- [[Calimshan]] imports: food, slaves, wizards;
- [[Calimshan]] exports: armor, books, gems, jewelry, leather goods, mercenaries, minor magic items, pearls, pottery, rare herbs, rope, ships, silk, spices, weapons, wine.

The Whale Campaign material further establishes active trade in silk, spices, gold, ivory, gems, furs, wine, tea, weapons, exotic goods, whale oil and regional shipping routes. Those records are campaign-canon unless separately verified against an official source.

## Market Roles

### [[Waterdeep]] — WDEX

**Role:** central consumption, processing, finance and redistribution hub.

Seed behaviours:

- deep liquidity;
- highest information quality among MVP markets;
- imports major quantities of food and raw material;
- manufactures/refines significant finished goods;
- strong luxury demand;
- largest warehouse capacity.

**Provenance:** mixture of FR canon and inference; exact commodity profile to be source-audited.

### [[Baldur's Gate]] — BGEX

**Role:** river/sea transshipment hub connecting the [[Chionthar]] hinterland to coastal shipping.

Seed behaviours:

- strong food and manufactured-goods throughput;
- meaningful weapons/textile trade;
- strong warehousing and re-export;
- moderate black-market leakage.

**Provenance:** campaign-canon + inference pending source audit.

### [[Athkatla]] — ATHEX

**Role:** southern capital market and agricultural/manufacturing export hub for [[Amn]].

Seed behaviours:

- large grain surplus;
- wine/ale production;
- metal/manufactured exports;
- aggressive merchant capital;
- relatively sophisticated information network.

**Provenance:** partially FR canon at regional level; city-level allocations are inference until audited.

### [[Calimport]] — CALEX

**Role:** southern high-volume manufacturing, shipbuilding and luxury export hub.

Explicit regional canon for [[Calimshan]] supports food imports and exports including armor, books, gems, jewelry, leather goods, mercenaries, minor magic items, pearls, pottery, rare herbs, rope, ships, silk, spices, weapons and wine.

Seed behaviours:

- structural food deficit;
- strong manufactured/luxury surplus;
- strong rope/shipbuilding demand and production;
- high-value goods liquidity;
- significant long-distance trade.

### [[Neverwinter]] — NWEX

**Role:** northern craft/industrial market with access to [[Neverwinter Wood]] and northern resources.

Regional [[Sword Coast North]] canon specifically lists Neverwinter's crafts among exports.

Seed behaviours:

- timber availability;
- craft/manufacturing output;
- construction demand;
- moderate luxury import demand;
- smaller liquidity than WDEX.

### [[Luskan]] — LUSEX

**Role:** northern gateway, high-risk maritime market, access point for regional resources and illicit redistribution.

Seed behaviours:

- exposure to northern timber/metals/furs;
- expensive southern luxuries/spices;
- higher route-risk premium;
- smaller formal market depth;
- larger illicit-market share.

## Commodity Catalogue v0

| ID | Commodity | Base unit | Family | Perishability | Primary MVP source tendency | Notes |
|---|---|---|---|---|---|---|
| CMD-GRAIN | Grain | kg | food | medium | Athkatla/BG hinterland | staple benchmark |
| CMD-FISH | Fresh Fish | kg | food | high | coastal markets | decay test |
| CMD-SALTFISH | Salted Fish | kg | food | low | coast/north | processing later |
| CMD-LIVESTOCK | Livestock | head-equivalent | food | special | southern/hinterland | open representation question |
| CMD-SALT | Salt | kg | food/input | none | external/regional | preservation input |
| CMD-TIMBER | Timber | m3 | raw | none | Neverwinter/north | bulk logistics benchmark |
| CMD-SHIPTIMBER | Ship Timber | m3 | strategic | none | north | quality-sensitive |
| CMD-IRONORE | Iron Ore | kg | raw | none | northern hinterland | upstream chain |
| CMD-IRON | Iron Ingots | kg | industrial | none | processing centers | downstream chain |
| CMD-HIDES | Raw Hides | kg | raw | medium | hinterland/north | leather chain later |
| CMD-WOOL | Wool | kg | raw | low | hinterland | textile input |
| CMD-CLOTH | Cloth | bolt | manufactured | none | Waterdeep/BG/Amn | manufactured trade |
| CMD-ROPE | Rope | coil-equivalent | manufactured | none | Calimshan/coastal | maritime input |
| CMD-POTTERY | Pottery | crate | manufactured | none | urban centers | fragile cargo later |
| CMD-TOOLS | Tools | crate | manufactured | none | urban/industrial | construction demand |
| CMD-WEAPONS | Weapons | item-equivalent | strategic | none | Calimshan/north | military demand |
| CMD-ARMOR | Armor | item-equivalent | strategic | none | Calimshan | high mass/value |
| CMD-ALE | Ale | litre | consumable | low | Waterdeep/Amn | common consumption |
| CMD-WINE | Wine | litre | luxury | quality-ageing | Calimshan/Amn | quality later |
| CMD-SPICES | Spices | kg | luxury | low | Calimshan/external | high value/low mass |
| CMD-SILK | Silk | bolt | luxury | none | Calimshan/external | high value/low mass |
| CMD-GEMS | Gems | parcel | luxury | none | north/Calimshan | provenance/quality sensitive |
| CMD-WHALEOIL | Whale Oil | litre | maritime/luxury | low | whaling regions | Whale Campaign relevance |
| CMD-ALCHEMY | Alchemical Reagents | crate | specialist | variable | multiple | magic economy hook |

## Initial Route Graph v0

These are topology decisions, **not yet canonical distances**.

### Sea trunk

- [[Calimport]] <-> [[Athkatla]]
- [[Athkatla]] <-> [[Baldur's Gate]]
- [[Baldur's Gate]] <-> [[Waterdeep]]
- [[Waterdeep]] <-> [[Neverwinter]]
- [[Neverwinter]] <-> [[Luskan]]

### Additional direct sea links

- [[Athkatla]] <-> [[Waterdeep]]
- [[Baldur's Gate]] <-> [[Neverwinter]]
- [[Waterdeep]] <-> [[Luskan]]

Direct links permit merchants to choose between fewer port calls and potentially different risk/capacity profiles.

### Overland/river abstractions

MVP should expose at least:

- [[Baldur's Gate]] <-> Chionthar hinterland supply node;
- [[Athkatla]] <-> Amnian agricultural hinterland node;
- [[Neverwinter]] <-> northern timber/craft hinterland node;
- [[Luskan]] <-> northern mineral/resource hinterland node.

These hinterlands can initially be production nodes rather than player-visible exchanges.

## Market Depth Classes

Provisional classes:

- Tier 5: [[Waterdeep]];
- Tier 4: [[Athkatla]], [[Baldur's Gate]], [[Calimport]];
- Tier 3: [[Neverwinter]];
- Tier 2–3: [[Luskan]] depending on legal vs illicit market.

These classes affect liquidity and order slippage, not the fundamental value of commodities.

## Information Quality v0

Provisional public-price latency assumptions:

- same market: immediate;
- adjacent major port: 1–3 days without magic;
- two or more legs away: 3–10 days;
- [[OTC]]/[[DTC/DWTC (Deepwater Trading Company)]]: faction-specific improvements later;
- [[Whale Society]] intelligence: special nonhuman network later.

## Deliberately Missing From v0

Do not add yet:

- hundreds of minor settlements;
- futures/options;
- local coin exchange rates;
- detailed taxes by gate/harbour;
- weather fields;
- individual ship crews;
- detailed NPC households;
- magical teleportation markets;
- full Toril trade.

The seed is successful if it produces recognizable north/south commodity flows and market shocks across six exchanges.
