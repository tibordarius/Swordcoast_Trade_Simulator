# Sprint 5 Status — Merchant AI v1

**Sprint:** 5  
**Planned dates:** 2026-12-07 to 2026-12-20  
**Work started early:** 2026-09-29

## Goal

Create the first autonomous economic actor capable of observing two markets, estimating executable prices and costs, and selecting a positive risk-adjusted physical arbitrage trade.

## Decision Model v0

For each candidate route/quantity:

```text
expected revenue
- executable purchase cost
- freight
- expected cargo loss
= expected profit
```

Then:

```text
ROI = expected profit / deployed capital
```

A trade is accepted only if:

- route capacity is sufficient;
- merchant capital covers purchase + freight;
- expected profit is positive;
- expected ROI meets the merchant threshold.

## First Opportunity

Benchmark:

`ATHEX [[Athkatla]] -> WDEX [[Waterdeep]]`

100,000 kg grain-equivalent cargo:

```text
purchase             165,500,000 mcp
expected revenue     337,800,000 mcp
freight               31,500,000 mcp
expected risk loss     1,655,000 mcp
-----------------------------------
expected profit      139,145,000 mcp
ROI                        70.63%
```

This ROI is intentionally very high because the seed world starts with a deliberately exaggerated surplus/shortage spread. Calibration comes later; the test only proves directional behaviour.

The reverse WDEX -> ATHEX trade produces a large expected loss and is rejected.

## Arbitrage Convergence Test

A static benchmark repeatedly completes 100,000 kg transfers from Athkatla to Waterdeep and recalculates both markets after every completed shipment.

Result:

```text
accepted shipments: 3
initial accepted ROI: 70.63%
next rejected attempt ROI: 2.54%
origin stock: 650,000 kg
Waterdeep stock: 940,000 kg
```

The merchant therefore pushes the two markets toward each other and then stops once the spread no longer clears its 5% minimum ROI.

This is an important first emergent result: **the model reduces arbitrage without a scripted price convergence rule.**

## Sprint Cards

- [x] AI-001 Merchant capital constraint.
- [x] AI-002 Market observation structure.
- [x] AI-003 Opportunity scoring via expected P&L/ROI.
- [x] AI-004 Risk-adjusted expected loss.
- [x] AI-005 Shipment/purchase decision threshold.
- [x] AI-006 Profit impact represented in benchmark capital.
- [x] TEST-003 Static arbitrage convergence reference test.

## Deliberately Deferred

Merchant v1 still assumes clean market observations. The following remain later enhancements:

- stale information and confidence;
- competing simultaneous merchants;
- portfolio/capital allocation across many goods;
- faction preferences;
- warehouse speculation;
- debt/credit;
- route discovery;
- contracts;
- strategic market manipulation.

## Next Sprint

Sprint 6 should be a validation/balancing buffer rather than immediately expanding features. The current chain is now complete enough to stress:

```text
inventory -> price -> executable order -> freight -> risk -> destination sale -> arbitrage convergence
```
