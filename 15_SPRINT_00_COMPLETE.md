# Sprint 0 Completion — Economic Contract

**Sprint:** 0  
**Dates:** 2026-09-29 to 2026-10-11  
**Status:** Completed baseline on 2026-09-29; calibration work remains intentionally outside the exit gate.

## Sprint Goal

Freeze the first economic ontology, units, deterministic conventions, MVP markets, commodity catalogue and route topology so implementation can begin without each subsystem inventing its own meanings.

## Completed Cards

- [x] SPEC-001 Currency: integer copper pieces.
- [x] SPEC-002 Commodity quantities: commodity-specific semantic base units with integer sub-unit scale.
- [x] SPEC-003 Time: five-minute authoritative tick plus multi-rate subsystems.
- [x] SPEC-004 Inventory/reserve vocabulary.
- [x] SPEC-005 Market price state vocabulary.
- [x] SPEC-006 Provenance classes.
- [x] DATA-001 Six-market seed.
- [x] DATA-002 Twenty-four-commodity catalogue.
- [x] DATA-003 First route topology.
- [x] Sprint exit: hand-worked Athkatla -> Waterdeep grain flow.

## Machine-Readable Deliverables

- `seed/world_v0.json`
- `seed/markets_v0.json`
- `seed/commodities_v0.json`
- `seed/routes_v0.json`
- `tools/validate_seed.py`

Validator result:

```text
seed validation: PASS
markets=6 commodities=24 routes=8
tick_minutes=5 world_seed=12345
```

## Decisions Frozen for Sprint 1

1. Money is integer cp.
2. Commodity quantities are integer fixed-point values, but units differ by commodity.
3. Cargo mass/volume are separate from economic quantity.
4. One authoritative base tick is five in-world minutes.
5. Subsystems execute at different cadences.
6. Incoming cargo is not equivalent to on-hand stock.
7. Prices are outputs, never direct world-event inputs.
8. Source provenance is first-class data.
9. Same seed + simulation version + ordered inputs must replay identically.
10. WDEX, BGEX, ATHEX, CALEX, NWEX and LUSEX are the initial formal markets.

## Explicitly Not Frozen

The following remain calibration/research tasks rather than Sprint 0 blockers:

- exact populations;
- final production volumes;
- final reference prices for most commodities;
- exact sea distances and voyage durations;
- taxes and port fees;
- detailed spoilage rates;
- weather and piracy probability models.

## Exit Assessment

**PASS.** The economic vocabulary and minimum seed data are specific enough to implement the deterministic kernel and the first grain vertical slice.
