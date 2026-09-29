# Sprint 6 Validation Buffer

**Sprint:** 6  
**Planned dates:** 2026-12-21 to 2027-01-03  
**Work started early:** 2026-09-29

## Goal

Stop adding features long enough to test whether the first economic chain behaves coherently under sustained simulation.

## Integrated 120-Day Test

The reference model now combines:

```text
production
-> consumption
-> inventory
-> reserve-driven price
-> bid/ask
-> slippage
-> merchant decision
-> freight
-> in-transit delay
-> arrival
-> destination sale
-> new inventory state
```

No direct price manipulation is used.

### Benchmark economy

Engineering values only, not final Forgotten Realms calibration:

- ATHEX starts grain-rich;
- WDEX starts below target reserve;
- Athkatla has a large production surplus;
- Waterdeep has a structural grain deficit;
- voyage time is 18 days;
- merchant checks opportunities every six hours;
- each shipment moves 100,000 kg.

## Results

### No merchant flow

```text
Waterdeep final stock: 0
Waterdeep unmet demand: 3,560,000 kg
average WDEX fundamental: 4.912 cp/kg
Athkatla final stock: 7,550,000 kg
shipments: 0
```

### Merchant with only two concurrent shipments

```text
Waterdeep final stock: 0
Waterdeep unmet demand: 2,360,000 kg
average WDEX fundamental: 4.766 cp/kg
Athkatla final stock: 6,150,000 kg
shipments: 14
```

### Merchant with eight concurrent shipments

```text
Waterdeep final stock: 1,068,750 kg
Waterdeep unmet demand: 428,750 kg
average WDEX fundamental: 3.538 cp/kg
Athkatla final stock: 2,550,000 kg
shipments: 50
```

## Important Finding

The original validation assertion expected any merchant activity to leave Waterdeep with more final stock. That assumption failed under the two-slot logistics constraint.

This is not treated as a bug in the simulation.

It reveals a real economic mechanism:

> **available regional surplus is not enough; sufficient transport throughput is also required.**

Athkatla can sit on millions of kilograms of grain while Waterdeep experiences shortages if capital, vessels, route throughput or voyage time prevent enough cargo from moving.

This is exactly the kind of emergent result the project is intended to produce.

## Validation Outcomes

- [x] deterministic kernel oracle passes;
- [x] conservation of goods passes;
- [x] unmet demand accounting passes;
- [x] scarcity pricing is monotonic;
- [x] known incoming supply reduces scarcity pressure;
- [x] larger orders suffer more slippage;
- [x] shipment arrival is idempotent;
- [x] merchant rejects negative arbitrage;
- [x] repeated arbitrage narrows the spread;
- [x] merchant activity lowers Waterdeep unmet demand;
- [x] merchant activity lowers average scarcity price;
- [x] increasing logistics throughput improves the shortage further;
- [x] no benchmark produces negative physical stock or merchant capital.

## Reference Suite

`tools/run_reference_suite.py` executes Sprint 1 through Sprint 6 reference tests as one regression suite.

Current result:

```text
REFERENCE SUITE: PASS
```

## Calibration Warning

The model is mechanically coherent enough to continue, but none of the benchmark production volumes, route capacity, voyage cost, elasticity coefficients or starting inventories should yet be treated as final Forgotten Realms economic facts.

Sprint 7 persistence should store these as data/configuration, not hardcode them into engine logic.

## Next Architecture Step

The next planned phase is persistence:

- PostgreSQL operational schema;
- PostGIS-ready geographic entities;
- migrations;
- event log;
- snapshots;
- seed import.

However, Rust compilation remains the outstanding technical gate before calling the native engine itself validated.
