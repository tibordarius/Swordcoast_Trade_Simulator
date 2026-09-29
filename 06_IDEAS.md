# Ideas & Experiments

Nothing in this file is committed unless promoted into [[07_DECISIONS]] and [[04_BACKLOG]].

## Exchange Ideas

- [[WDEX - Waterdeep Exchange]] composite index.
- [[BGEX - Baldur's Gate Exchange]].
- [[NWEX - Neverwinter Exchange]].
- [[LUSEX - Luskan Exchange]].
- [[ATHEX - Athkatla Exchange]].
- [[CALEX - Calimport Exchange]].
- [[SCTX - Sword Coast Trade Index]].
- [[SCFI - Sword Coast Freight Index]].
- sector indices: Food, Maritime, Industrial, Luxury, Arcane.

## Information Markets

- price sheets have latency by distance;
- merchant houses can purchase faster intelligence;
- [[Whale Society]] dolphins can provide unusually fast maritime information;
- [[OOS]] creates bizarre specialist demand spikes;
- rumours can be wrong while the underlying state remains true;
- player information advantage could exist before public market repricing.

## Trading Features

- warehouse receipts;
- forward delivery contracts;
- charter contracts;
- limit orders;
- cargo insurance;
- convoy shares;
- salvage rights;
- ship financing;
- letters of credit;
- bills of exchange;
- merchant reputation and credit limits;
- customs bonds;
- smuggling contracts.

## Map Features

- animated trade-line thickness by volume;
- colour/intensity by value or price pressure;
- click a route to inspect carried commodities;
- click a city to see days-of-supply by commodity;
- commodity origin Sankey view;
- map replay through time;
- weather overlay;
- piracy heatmap;
- navy patrol coverage;
- regional harvest quality;
- fish migration and whaling zones;
- undersea Whale Society routes invisible to normal surface traders.

## Economic Replay

Rewind to any tick and replay the market on the map. Display the sequence from shock to consequence:

`event -> route change -> inventory change -> order flow -> price move -> merchant response -> shipment arrival`

## "Why?" Causal Explorer

Click a price movement and recursively inspect causes. Example:

`Waterdeep grain +23%`

-> `Amnian imports -38%`

-> `three delayed convoys`

-> `piracy + storm`

-> specific pirate event or campaign session.

## Scenario Lab

Fork the world to test:

- Luskan blockade;
- Mirabar mine collapse;
- whale-oil embargo;
- Royal Navy mobilisation;
- magical plague;
- new teleport corridor;
- major pirate victory;
- destruction of an OTC warehouse network.

Run one deterministic branch or many seeded Monte Carlo branches.

## Offline Campaign Edition

Possible later stack:

- PGlite;
- Rust/WASM simulation;
- DuckDB-Wasm;
- PMTiles;
- browser IndexedDB/OPFS.

Goal: full campaign simulator on a laptop without internet.

## AI Assistance

AI may:

- extract lore facts from sourcebooks;
- propose structured economic records;
- translate session notes into candidate simulation events;
- explain complex market chains in natural language;
- identify suspicious/outlier simulation behaviour.

AI should not be the authoritative price engine.
