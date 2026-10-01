# PH4-006 Acceptance

- [ ] Production recipe/site/batch IDs and types exist.
- [ ] Starting a batch reserves input through a balanced transaction.
- [ ] Production completion is scheduler-driven.
- [ ] Completion consumes WIP and creates output through explicit conversion postings.
- [ ] Batch completes exactly once.
- [ ] Population cohort consumption is scheduler-driven and recurring.
- [ ] Insufficient stock records unmet demand without negative inventory.
- [ ] Consumption records demanded/consumed/unmet quantities.
- [ ] AdvanceTo processing is atomic on failure.
- [ ] Replay produces identical economy state/hash.
- [ ] Snapshot roundtrip preserves production/population/scheduler state.
- [ ] Existing ScenarioPack/ledger/scheduler tests remain green.
- [ ] Full GitHub CI passes.
