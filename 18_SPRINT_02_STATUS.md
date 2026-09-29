# Sprint 2 Status — Supply, Consumption & Inventory

**Sprint:** 2  
**Planned dates:** 2026-10-26 to 2026-11-08  
**Work started early:** 2026-09-29

## Goal

Prove the physical commodity ledger before adding prices, merchants or logistics.

## Implemented Reference Behaviour

The first benchmark is [[Athkatla]] [[Grain]]. Quantities use milli-kg internally.

Engineering seed:

- starting stock: 700,000 kg;
- production: 30,000 kg/day;
- consumption: 25,000 kg/day;
- target reserve: 700,000 kg;
- spoilage hook: 500 ppm/day (engineering benchmark, not lore).

## Completed Slices

### SIM-001 Production site mechanics

A fixed daily rate is distributed across 24 hourly updates without integer drift using cumulative integer allocation.

### SIM-002 Consumption sector mechanics

Requested demand is separated from fulfilled consumption. Unmet demand is accumulated rather than allowing inventory to go negative.

### SIM-003 Inventory ledger

Implemented fields:

- on hand;
- reserved;
- target reserve;
- produced;
- consumed;
- spoiled;
- unmet demand;
- derived available.

### SIM-004 Reserve target

Target reserve exists in the ledger. Reserve-based pricing belongs to Sprint 3.

### SIM-005 Spoilage hook

Implemented deterministic daily spoilage in parts-per-million. Final grain spoilage calibration remains research work.

### TEST-002 Conservation of goods

Reference invariant:

```text
initial_stock + produced
=
on_hand + consumed + spoiled
```

Demand invariant:

```text
requested_demand
=
consumed + unmet_demand
```

Both pass.

## Reference Results

### Normal 30-day benchmark

```text
on_hand_mkg=838418883
produced_mkg=900000000
consumed_mkg=750000000
spoiled_mkg=11581117
unmet_mkg=0
```

### Ten-day shortage benchmark

```text
on_hand_mkg=0
produced_mkg=50000000
consumed_mkg=99980008
spoiled_mkg=19992
unmet_mkg=150019992
```

This proves stock never becomes negative and unsatisfied consumption remains visible to the later demand/price system.

## Rust Source Added

- `inventory.rs`
- `rate.rs`
- `tests/inventory.rs`

These mirror the passing Python oracle but remain uncompiled until a Rust-capable environment is available.

## Next Slice

Sprint 3 can now model price as a response to:

- reserve coverage;
- unmet demand;
- current stock trend;
- expected incoming supply;
- liquidity/slippage.

The first test should compare normal, surplus and shortage cases without any random price assignment.
