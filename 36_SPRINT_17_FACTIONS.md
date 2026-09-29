# [[Sprint 17 — Advanced Merchant Factions]]

## Status

**Reference-complete.**

## Principle

Faction economics are data-driven. The engine does not contain logic such as `if faction == OTC`.

A trader receives a strategy profile describing:

- minimum ROI;
- information freshness;
- maximum route reach;
- preference for bulk cargo;
- preference for value density;
- distance preference;
- controlled-route preference;
- remote-market preference;
- warehouse patience;
- information quality;
- risk tolerance.

Campaign factions are then bound to generic profiles.

## Current profiles

### [[OTC]]

Uses the **Vertically Integrated Bulk Trader** profile. It prefers high-volume cargoes, controlled routes and warehousing, and accepts lower margins where scale or route control compensates.

### [[DTC/DWTC (Deepwater Trading Company)]]

Uses the **Long-Range Sparse Network Trader** profile. It has better remote information, greater route/risk tolerance, and a stronger preference for high-value cargo that justifies long voyages.

### Independent merchants

Prefer nearer, clearer arbitrage with less strategic bias.

## Benchmark result

Given the same four opportunities:

```text
Independent → local Baldur's Gate / Waterdeep grain
OTC         → controlled bulk timber lane
DTC         → remote high-value silk opportunity
```

The remote opportunity is rejected by the OTC profile because it exceeds its configured reach, while DTC can observe and evaluate it.
