# sim-kernel-v2 Module Contract

## Purpose
Future authoritative deterministic economic simulation kernel.

## Owns
Typed simulation primitives and, in later stages, scheduler, transactions, ledgers, reducer, market, information, logistics, population, production, finance, and shocks.

## Does not own
Source/canon interpretation, TorilGIS provenance authoring, frontend rendering, or direct persistence/UI orchestration.

## Public API
Currently foundational value/ID/time/RNG/hash types only.

## Dependencies
Keep dependencies minimal. Domain layers may depend inward on foundational modules.

## Forbidden dependencies
- v1 sim-core internals
- frontend code
- Google Drive/TorilGIS source crawlers
- mutable application/database handles in domain primitives

## Invariants
- authoritative primitive arithmetic is integral/fixed-point;
- deterministic utilities do not depend on wall-clock time or thread scheduling;
- stable IDs serialize predictably.

## Focused tests
cargo test -p sim-kernel-v2
