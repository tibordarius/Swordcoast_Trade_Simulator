# ADR-0003: Fixed-Point Authoritative Economics

Status: Accepted

## Context
The v1 model stores much currency as integer copper pieces but still uses floating point for authoritative price/slippage/risk calculations. Sub-copper prices and cross-runtime deterministic replay require a stronger contract.

## Decision
v2 authoritative monetary and physical quantities use integral/fixed-point domain types. Floating point is non-authoritative unless a later ADR explicitly permits it.

## Consequences
- Money, quantity, ratios, and unit prices are distinct types.
- Rounding rules must be explicit and tested.
- Overflow tests are required before final integer widths are frozen.
