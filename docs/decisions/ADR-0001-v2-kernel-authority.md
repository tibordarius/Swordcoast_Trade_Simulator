# ADR-0001: v2 Kernel Authority

Status: Accepted

## Context
Phase 3 found cross-cutting v1 defects in inventory mutation, merchant shipment identity, fixed-point pricing, stale information, and multi-leg logistics.

## Decision
Build `sim-kernel-v2` as the future authoritative simulation kernel while preserving v1 as a regression/reference implementation.

## Consequences
- New structural work targets v2.
- v1 remains runnable and is not casually refactored.
- Useful v1 concepts/tests may be ported selectively.
- Numerical parity is required only where explicitly specified; invariants and qualitative behavior are more important.
