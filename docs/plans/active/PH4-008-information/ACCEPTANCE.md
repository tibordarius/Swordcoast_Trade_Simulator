# PH4-008 Acceptance

- [ ] MarketObservationId is typed and stable.
- [ ] Knowledge actors are explicitly registered.
- [ ] Dispatch rejects unknown actors and unknown market listings.
- [ ] Observation captures quote/tick at dispatch time.
- [ ] Observation is invisible to actor before delivery.
- [ ] Delivery occurs at deterministic scheduled tick.
- [ ] KnowledgeView reports observation age and transport delay.
- [ ] Market changes during transit do not change the in-flight observation.
- [ ] Late-arriving older observations do not overwrite newer knowledge.
- [ ] Duplicate observation IDs are rejected.
- [ ] Information delivery does not create market trades or mutate balances.
- [ ] Replay and snapshot preserve pending and delivered information state.
- [ ] Snapshot schema advances for information state/payloads.
- [ ] Existing market/production/ScenarioPack tests remain green.
- [ ] Full GitHub CI passes.
