# PH4-004 Acceptance

- [ ] EventId is a stable typed ID.
- [ ] Events cannot be scheduled before current SimTick.
- [ ] Same-tick events order by domain phase, then insertion sequence.
- [ ] Cancelled events do not fire.
- [ ] Rescheduling the same EventId invalidates its older queued generation.
- [ ] AdvanceTo drains all valid due events through the requested tick.
- [ ] Snapshot roundtrip preserves scheduler state.
- [ ] Deterministic replay produces the same fired-event order and state hash.
- [ ] Existing ledger/replay tests remain green.
- [ ] Full GitHub CI passes.
