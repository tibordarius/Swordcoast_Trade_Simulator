# PH4-006 Acceptance

- [ ] Recipe/site/cohort/batch IDs are typed and stable.
- [ ] Starting a production batch moves input from Holding inventory into WIP through EconomicTransaction.
- [ ] Production completion occurs only from its scheduled event.
- [ ] Completion consumes WIP and creates output through an explicit SourceOrSink account.
- [ ] A production batch cannot complete twice.
- [ ] Insufficient input rejects batch start without partial mutation.
- [ ] Cohort consumption repeats on a deterministic interval.
- [ ] Consumption serves available stock only and records unmet quantity separately.
- [ ] Unmet demand is per-cycle data, not an ever-growing permanent price-pressure accumulator.
- [ ] AdvanceTo is atomic if any due event fails.
- [ ] Replay and snapshot roundtrip preserve production/population state.
- [ ] Existing ScenarioPack/ledger/scheduler tests remain green.
- [ ] Full GitHub CI passes.
