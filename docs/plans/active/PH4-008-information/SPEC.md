# PH4-008 Delayed Information and Actor Knowledge

## Objective
Introduce actor-local market knowledge so economic actors act on observations that may be delayed and stale rather than reading global market truth.

## Scope
- typed MarketObservationId;
- explicit knowledge actors;
- MarketObservation snapshot of a MarketQuote at observation time;
- delayed delivery through the deterministic scheduler;
- actor-local KnowledgeView;
- freshness/age metadata;
- latest-observation selection that never regresses when older information arrives later;
- delivered-observation history;
- duplicate observation protection;
- replay/snapshot coverage.

## Non-goals
- merchant decision AI;
- rumor mutation/noise;
- information pricing;
- social networks;
- multi-hop message routing;
- newspapers/signals/spells;
- route-derived communication delay;
- actor physical location.

## Invariants
- an observation captures market truth at dispatch time, not delivery time;
- undelivered observations are absent from actor knowledge;
- market changes after dispatch do not rewrite the observation;
- older observations delivered later never overwrite newer observed knowledge;
- actor KnowledgeView contains only delivered actor-local observations;
- dispatching/receiving information does not mutate market truth;
- duplicate MarketObservationId is rejected;
- scheduler/replay/snapshot remain deterministic.
