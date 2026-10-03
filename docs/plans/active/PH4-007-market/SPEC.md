# PH4-007 Fixed-Point Market v2

## Objective
Add the first explainable posted-price market on top of real inventory and recent physical consumption.

## Scope
- typed MarketTradeId;
- MarketListing state per market/commodity;
- fixed-point quote derivation;
- bounded scarcity signal from local stock versus target stock;
- bounded recent-unmet-demand pressure from cohort consumption records;
- bid/ask spread at sub-copper precision;
- deterministic market-depth slippage;
- actual buy/sell settlement through EconomicTransaction;
- trade history containing real executions only;
- structured PriceExplanation;
- replay/snapshot coverage.

## Non-goals
- order book;
- credit;
- merchant AI;
- stale observations;
- known inbound shipments;
- institution reserves;
- market-to-market arbitrage;
- dynamic price inertia/EMA;
- ScenarioPack market calibration fields.

## Invariants
- quote calculation is read-only;
- quote refresh does not create trade history;
- every trade moves both goods and money through EconomicTransaction;
- trade quantity and price are positive;
- Holding accounts cannot go negative;
- recent unmet demand is window-bounded, not cumulative forever;
- bid < ask for positive spread configuration even at low copper prices;
- execution is deterministic and replayable.
