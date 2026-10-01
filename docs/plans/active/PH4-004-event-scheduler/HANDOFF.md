# PH4-004 Handoff

## Goal
Implement deterministic future-event scheduling without introducing domain-specific event behavior.

## Status
REVIEW

## Branch
phase4/v2-event-scheduler

## Completed
- EventId;
- EventDomain with explicit phase priority;
- ScheduledEvent and FiredEvent;
- deterministic ordering key;
- event generations and active-generation tracking;
- ScheduleEvent and CancelEvent commands;
- past-event rejection;
- reschedule invalidation;
- cancellation invalidation;
- AdvanceTo due-event draining;
- scheduler snapshot/replay state;
- integration tests for order, cancellation, reschedule, replay, and snapshots;
- full repository CI verification.

## Tests last run
GitHub Actions run 36876220349: all jobs successful.

## Current failure / blocker
None.

## Important files
- code/crates/sim-kernel-v2/src/scheduler.rs
- code/crates/sim-kernel-v2/src/state.rs
- code/crates/sim-kernel-v2/src/reducer.rs
- code/crates/sim-kernel-v2/tests/event_scheduler.rs

## Next action
Review branch diff, open PR, and merge if scope remains limited to scheduler semantics.

## Do not redo
Do not add production/logistics payload behavior here. Future domain systems should dispatch from this proven scheduler rather than create their own time loops.
