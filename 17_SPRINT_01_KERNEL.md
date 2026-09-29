# Sprint 1 Kernel Design — Implemented Skeleton

## Purpose

Document the exact deterministic mechanisms now represented in both the Python reference oracle and the Rust source skeleton.

## Named RNG Streams

A world seed is not consumed directly by subsystems. Each subsystem receives a deterministic derived seed:

```text
derived_seed = FNV1a64(world_seed_le_bytes || namespace_utf8)
```

Initial namespaces:

- `production`
- `events`

Adding a new subsystem stream does not change the `production` stream.

### Why this matters

If piracy later uses random draws, adding a piracy feature must not retroactively change every historical harvest result. Named streams make random evolution locally stable.

## RNG Algorithm v0

SplitMix64 is implemented directly in the kernel rather than delegated to a crate.

This is not selected for cryptographic quality. It is selected because the algorithm is small, fast and exactly reproducible across Rust, Python and future WASM/native builds.

Changing the RNG algorithm later is a simulation-version change.

## Scheduler

Events are ordered by:

1. `due_tick`
2. `sequence`
3. `kind`

The priority queue returns the earliest deterministic event first.

The first recurring event is a production heartbeat every 12 base ticks, equivalent to one in-world hour.

## Delta Commit

Subsystem handlers do not directly mutate shared economic aggregates in the intended mature design. They emit deltas.

Deltas sort by:

```text
(priority, entity_id, kind, sequence)
```

and are then committed in that stable order.

The current skeleton has one delta kind, but the ordering contract exists before multiple systems are added.

## Golden Oracle

Python reference result:

```text
seed=12345
ticks=10000
hash=ea50aa8c6fbd16cb
production_signal=-19
next_sequence=1668
```

Checks passed:

- repeated same seed gives identical output;
- different seed diverges;
- consuming 1,000 values from the unrelated `events` RNG does not change production results;
- scheduled events remain ordered;
- deltas commit deterministically.

## Remaining Sprint 1 Work

- Rust compile/test in a Rust-capable environment;
- expand typed IDs beyond generic `EntityId`;
- add scheduler cancellation/rescheduling semantics only when a real use case appears;
- add profiling after the kernel contains enough work to measure meaningfully.

## Sprint 2 Handoff

Sprint 2 can now introduce real economic state behind the delta boundary:

- `ProductionSite`;
- `ConsumptionSector`;
- `InventoryLedger`;
- grain production/consumption;
- conservation-of-goods invariant.
