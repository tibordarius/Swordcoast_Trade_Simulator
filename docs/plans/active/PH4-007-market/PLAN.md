# PH4-007 Plan

1. Add MarketTradeId.
2. Extend fixed-point UnitPrice helpers for checked PPM/basis-point scaling.
3. Add market module types:
   - MarketSide;
   - MarketListing;
   - MarketQuote;
   - PriceExplanation;
   - MarketTrade.
4. Add market listing/trade history state to WorldState.
5. Add RegisterMarketListing and ExecuteMarketTrade commands.
6. Derive quote from:
   - local inventory;
   - target stock;
   - recent requested/unmet consumption within a bounded window;
   - fixed spread configuration.
7. Apply deterministic depth/slippage at execution.
8. Settle trade via one EconomicTransaction.
9. Add low-price, shortage, slippage, no-synthetic-trade, insufficient-cash/inventory, replay, and snapshot tests.
10. Run full repository CI.

Compatibility:
- no direct inventory or money mutation;
- production/consumption remain physical truth;
- ScenarioPack boundary unchanged;
- v1 untouched.
