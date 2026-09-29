# Sprint 1 Status — Deterministic Simulation Skeleton

**Sprint:** 1  
**Planned dates:** 2026-10-12 to 2026-10-25  
**Work started early:** 2026-09-29

## Sprint Goal

Create a minimal deterministic engine capable of advancing a five-minute world clock, consuming seeded pseudo-randomness only at explicit subsystem boundaries, and producing a stable state hash after a known number of ticks.

## Cards

### ENG-001 — Create Rust workspace

**Status:** Implemented, not compiled in current container.

Created:

- `code/Cargo.toml`
- `code/crates/sim-core`
- `code/apps/sim-cli`

The core intentionally has no third-party dependencies yet.

### ENG-002 — Simulation clock

**Status:** Implemented in `clock.rs`.

- five-minute default tick;
- checked tick increment;
- hour-boundary detection.

### ENG-003 — Stable IDs

**Status:** Initial implementation in `ids.rs`.

- `WorldId`;
- `EntityId`.

Typed domain IDs will be expanded as entities become real.

### ENG-004 — Seeded RNG streams

**Status:** Kernel implemented using SplitMix64.

Reason for first implementation:

- tiny;
- completely specified;
- trivial to reproduce across Rust/Python/WASM;
- no library-version dependency for deterministic replay.

This does not yet implement separate named subsystem streams. That is the next card slice.

### ENG-005 — Phase scheduler

**Status:** Minimal hourly cadence proved; generalized scheduler not yet implemented.

The reference world consumes RNG only at hour boundaries. This proves that visual/base ticks do not automatically consume randomness.

### ENG-006 — Deterministic delta commit

**Status:** Not started. Required before production/consumption systems mutate shared state.

### TEST-001 — Determinism golden test

**Status:** Reference oracle PASS; Rust test authored but cannot be compiled in current environment.

Independent Python result:

```text
seed=12345 ticks=10000 hash=4afdcd085b4f30b7 production_signal=72
repeat same seed hash=4afdcd085b4f30b7 production_signal=72
seed=54321 ticks=10000 hash=8a809d191a2ef34b production_signal=19
determinism reference: PASS
```

The Rust golden test asserts the same first result.

### OBS-001 — Tick profiling

**Status:** Not started. Add after scheduler exists; measuring the trivial kernel is not useful.

## Current Blocker

The available execution container has neither `rustc` nor `cargo` and DNS cannot resolve `sh.rustup.rs`. Therefore Rust compilation cannot be honestly verified here.

The project mitigates this by maintaining a dependency-free Rust kernel plus an independently executable Python oracle.

## Next Implementation Slice

1. named deterministic RNG streams derived from `world_seed + stable subsystem ID`;
2. event/phase scheduler;
3. delta buffer and deterministic commit ordering;
4. state hash covering scheduled events;
5. then begin Sprint 2 grain production/inventory reference model.
