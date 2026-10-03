# PH4-008 Acceptance

- [x] MarketObservationId is typed and stable.
- [x] Knowledge actors are explicitly registered.
- [x] Dispatch rejects unknown actors and unknown market listings.
- [x] Observation captures quote/tick at dispatch time.
- [x] Observation is invisible to actor before delivery.
- [x] Delivery occurs at deterministic scheduled tick.
- [x] KnowledgeView reports observation age and transport delay.
- [x] Market changes during transit do not change the in-flight observation.
- [x] Late-arriving older observations do not overwrite newer knowledge.
- [x] Duplicate observation IDs are rejected.
- [x] Information delivery does not create market trades or mutate balances.
- [x] Replay and snapshot preserve pending and delivered information state.
- [x] Snapshot schema advances for information state/payloads.
- [x] Existing market/production/ScenarioPack tests remain green.
- [x] Full GitHub CI passes.

## Evidence

GitHub Actions run 37111272952 passed:
- Rust;
- reference-model;
- DuckDB analytics;
- Postgres schema;
- v2 tiny ScenarioPack validation.
