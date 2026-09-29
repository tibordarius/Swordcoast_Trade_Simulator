# Sprint 3 Status — Pricing & Market Depth

**Sprint:** 3  
**Planned dates:** 2026-11-09 to 2026-11-22  
**Work started early:** 2026-09-29

## Goal

Make price an inspectable response to economic state rather than a random modifier.

## Important Model Correction

The original Sprint 0 contract stored money in integer copper pieces. Sprint 3 revealed that **unit quotations require sub-copper precision**. The hand-worked grain example itself uses values such as `1.75 cp/kg`.

### Revised rule

- authoritative monetary/accounting precision: **milli-copper pieces (`mcp`)**;
- `1 cp = 1,000 mcp`;
- `1 gp = 100,000 mcp`;
- no floating-point money or prices;
- physical coin display can still round/convert at the UI boundary.

This change is recorded as a superseding decision and should be reflected in future seed/schema versions.

## Pricing Model v0

Price pressures are represented in integer basis points.

```text
fundamental = reference_price × total_pressure_multiplier
```

Current causal components:

- reserve/stock pressure;
- recent unmet demand;
- effective known incoming supply;
- route/replacement risk input.

The model clamps extreme multipliers while calibration remains immature.

## Benchmark Behaviour

Reference price: `2,000 mcp/kg` = `2 cp/kg`.

| State | Fundamental | Bid | Ask |
|---|---:|---:|---:|
| Surplus | 1,000 mcp | 996 | 1,004 |
| Normal reserve | 2,000 | 1,992 | 2,008 |
| 50% reserve shortage | 3,500 | 3,486 | 3,514 |
| Severe shortage example | 5,260 | 5,239 | 5,281 |

The exact elasticity is an engineering baseline, not final economic calibration.

## Explainability

Every calculation returns causal components rather than only the resulting price.

Severe-shortage example:

```text
reserve      +13,927 bps
unmet demand  +2,500 bps
incoming        -428 bps
risk            +300 bps
--------------------------------
total multiplier 26,299 bps
```

That decomposition is the basis of the later [[Why Did This Move?]] interface.

## Liquidity and Slippage v0

Markets expose a depth quantity. A market order creates adverse impact according to size relative to depth.

Benchmark at a normal ATHEX-like ask of `2,008 mcp` with depth `40,000` quantity units:

- 10,000-unit buy: average `2,013 mcp`;
- 100,000-unit buy: average `2,058 mcp`.

Large orders therefore receive worse execution even when fundamental value does not change during the execution itself.

## Sprint Cards

- [x] MKT-001 Base/reference price precision and metadata contract.
- [x] MKT-002 Initial scarcity function.
- [x] MKT-003 Demand pressure through recent unmet demand.
- [x] MKT-004 Bid/ask spread by liquidity tier.
- [x] MKT-005 Mathematical liquidity/depth representation.
- [x] MKT-006 Slippage benchmark.
- [x] MKT-007 Price explanation component structure.

## Tests Passing in Reference Oracle

```text
price monotonicity: PASS
incoming supply expectation: PASS
market slippage: PASS
integer pricing: PASS
```

Rust source mirrors these tests but remains uncompiled in this container.

## Next Sprint

Sprint 4 now has everything needed to make price differences persist long enough for logistics to matter:

- route graph;
- capacity;
- cost;
- shipment state;
- ETA;
- arrival;
- delay/loss risk;
- derived visual position.
