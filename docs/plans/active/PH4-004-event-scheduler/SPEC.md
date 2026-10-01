# PH4-004 Deterministic Event Scheduler

## Objective
Add a deterministic future-event queue so later production, logistics, information, and shock systems can schedule work instead of polling every world object.

## Scope
- EventId;
- EventDomain with explicit phase ordering;
- ScheduledEvent;
- generation/version invalidation;
- deterministic same-tick sequence;
- schedule and cancel commands;
- AdvanceTo drains due valid events in deterministic order;
- fired-event history for verification;
- snapshot/replay coverage.

## Non-goals
- production batches;
- shipment arrival behavior;
- event payload dispatch into domain transactions;
- recurring events;
- optimized calendar queues/timing wheels.

## Invariants
- events cannot be scheduled in the past;
- same tick orders by domain phase then deterministic insertion sequence;
- cancelling/rescheduling invalidates older queued generations;
- stale events never fire;
- AdvanceTo remains atomic from the external command perspective;
- scheduler state survives snapshot/replay.
