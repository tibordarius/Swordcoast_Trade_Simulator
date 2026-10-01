# PH4-004 Result

## Outcome
Deterministic future-event scheduling implemented.

## Added
- EventId;
- EventDomain with explicit phase priority;
- scheduled and fired event records;
- deterministic same-tick ordering by domain phase then insertion sequence;
- version/generation invalidation;
- ScheduleEvent and CancelEvent commands;
- past-event rejection;
- rescheduling of the same EventId;
- AdvanceTo due-event drain;
- snapshot/replay coverage.

## Verification
GitHub Actions run 36876220349 passed all jobs.

## Architectural result
The v2 kernel can now represent future work without scanning every object every tick. Domain-specific event payload execution is intentionally deferred.

## Follow-up
PH4-005 should add the minimal ScenarioPack v2 loader/validator and a tiny executable world fixture.
