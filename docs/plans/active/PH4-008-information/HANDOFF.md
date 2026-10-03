# PH4-008 Handoff

## Goal
Add delayed actor-local market knowledge without merchant decision logic.

## Status
REVIEW

## Branch
`phase4/v2-information`

## Completed
- typed MarketObservationId;
- explicit knowledge-actor registration;
- MarketObservation capturing immutable dispatch-time MarketQuote;
- delayed scheduler delivery;
- KnowledgeView with age/freshness helpers;
- actor-isolated knowledge;
- delivery history;
- duplicate observation protection;
- stale late-arrival protection based on observed time/dispatch sequence;
- boxed information event payload to keep scheduler enum compact;
- snapshot schema version 6;
- replay/snapshot pending-message coverage.

## Tests last run
GitHub Actions run 37111272952: all jobs successful.

## Current failure / blocker
None.

## Important files
- code/crates/sim-kernel-v2/src/information.rs
- code/crates/sim-kernel-v2/src/scheduler.rs
- code/crates/sim-kernel-v2/src/reducer.rs
- code/crates/sim-kernel-v2/src/state.rs
- code/crates/sim-kernel-v2/tests/information.rs

## Next action
Review branch diff, open PR, and merge if scope remains limited to information/knowledge mechanics.

## Do not redo
Do not add merchant arbitrage scanning or restore omniscient global quote access for future merchant agents. PH4-010 must consume KnowledgeView.
