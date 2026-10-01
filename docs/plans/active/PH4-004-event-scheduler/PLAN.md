# PH4-004 Plan

1. Add EventId.
2. Add scheduler module with EventDomain, ScheduledEvent, FiredEvent, and private ordering key.
3. Add scheduler state to WorldState.
4. Add ScheduleEvent and CancelEvent commands.
5. Extend WorldReducer validation/commit logic.
6. Make AdvanceTo drain due events before committing target tick.
7. Bump snapshot schema version.
8. Add ordering, cancellation, reschedule, replay, and snapshot tests.
9. Run focused and full repository gates.

Compatibility:
- no domain event side effects yet;
- existing ledger/replay behavior remains unchanged.
