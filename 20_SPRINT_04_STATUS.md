# Sprint 4 Status — Logistics

**Sprint:** 4  
**Planned dates:** 2026-11-23 to 2026-12-06  
**Work started early:** 2026-09-29

## Goal

Make spatial arbitrage physically costly and slow. Goods must leave one inventory, exist in transit, and arrive exactly once.

## Logistics Benchmark

First route benchmark:

`[[Athkatla]] -> [[Waterdeep]]`

Engineering values, not final lore calibration:

- travel: 18 days = 5,184 five-minute ticks;
- daily capacity: 500,000 kg-equivalent in the benchmark dimension;
- variable grain transport cost: 0.300 cp/kg;
- fixed route/shipment cost: 1,500 cp;
- 100,000 kg shipment total cost: 31,500 cp.

The values were chosen to reproduce the scale of the existing hand-worked grain example, not to assert a canonical Faerûnian freight tariff.

## State Transition

```text
AVAILABLE AT ORIGIN
      ↓ reserve
RESERVED AT ORIGIN
      ↓ load/depart
IN TRANSIT
      ↓ ETA / event
ARRIVED or LOST
```

Departure removes goods from origin on-hand inventory. Destination inventory changes only on arrival.

## Implemented Mechanics

### LOGI-001 Route object

Contains travel ticks, daily capacity, variable cost and fixed cost. Geographic route edges remain a later PostGIS/pgRouting concern.

### LOGI-002 Capacity hook

Routes can reject a shipment larger than their configured capacity. Shared capacity across many simultaneous shipments remains a later allocator.

### LOGI-003 Transport cost

```text
fixed_cost + quantity × variable_cost
```

using integer milli-copper arithmetic.

### LOGI-004 Shipment lifecycle

Implemented:

- reserved;
- in transit;
- arrived;
- lost.

### LOGI-005 ETA

Stored as deterministic departure and arrival ticks.

### LOGI-006 Risk/loss hook

A shipment can enter `Lost` state and can no longer arrive. Probability/event resolution will use the named RNG/event framework rather than being embedded in the shipment object.

### LOGI-007 Derived map position

Shipment progress is a derived integer basis-point value from `0` to `10,000` based on current tick, departure and ETA. No position writes are necessary while travelling.

## Reference Checks

```text
route=SEA-ATH-WD travel_ticks=5184 cost_mcp=31500000
shipment_eta=105184 progress_final_bps=10000
reservation/departure accounting: PASS
derived position: PASS
arrival idempotency: PASS
route capacity hook: PASS
loss state hook: PASS
```

## Important Invariant

Calling arrival twice must not duplicate cargo. The benchmark destination moves from 720,000 kg to 820,000 kg once and remains there on subsequent arrival calls.

## Remaining Logistics Work

- shared route-capacity allocation across many shipments;
- route/path alternatives;
- delay events;
- probability calibration for storms/piracy;
- port loading time;
- vessel mass/volume constraints;
- PostGIS geometry;
- route-cache invalidation.

These are not blockers for Sprint 5 merchant arbitrage.
