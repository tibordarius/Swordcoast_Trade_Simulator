# PH4-008 Result

## Outcome
The v2 kernel now has delayed, actor-local market knowledge instead of requiring future agents to read global market truth.

## Added
- typed MarketObservationId;
- explicit knowledge actors;
- MarketObservation with immutable dispatch-time MarketQuote;
- deterministic information delivery through scheduler events;
- KnowledgeView;
- observation age and transport-delay helpers;
- delivered-observation audit history;
- duplicate observation protection;
- freshness ordering that prevents older late arrivals from replacing newer observed knowledge;
- boxed information payloads to keep the scheduler compact;
- snapshot format version 6.

## Behavioral result
A market can change while information is travelling. The receiving actor still gets the old quote that was actually observed at dispatch time.

Later delivery does not imply newer information.

## Verification
GitHub Actions run 37111272952 passed all jobs.

## Follow-up
PH4-009 should add physical multi-leg routing/logistics. PH4-010 merchant behavior must then combine KnowledgeView with those physical routes rather than scanning global market truth.
