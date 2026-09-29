# [[Sprint 16 — Production Chains]]

## Status

**Reference-complete.**

## Delivered

- `production_recipe`, `recipe_input`, `recipe_output`, `production_site`, `site_recipe`, `site_inventory` and `production_job` persistence schema;
- seed recipes for [[Iron Ore]] → [[Iron Ingots]] → [[Tools]] / [[Weapons]];
- additional calibration recipes for preserving fish and weaving cloth;
- site-level inventory buffers;
- explicit recipe transformations rather than implicit creation/destruction of commodities;
- delayed upstream-shock propagation test.

## Validation result

A temporary mine shutdown does not stop a downstream forge instantly. Existing site stock and material already in transit absorb the shock first.

Reference result:

```text
baseline tool output: 60,000 milli-crates
shock tool output:    50,000 milli-crates
first forge shortage: day 19
mine shock begins:    day 10
zero-output days:     10
```

This establishes the causal behavior needed for future chains such as timber → ship components, wool → cloth, food preservation, shipbuilding and alchemical production.

## Provenance warning

Recipe ratios are simulation calibration values, **not Forgotten Realms canon**.
