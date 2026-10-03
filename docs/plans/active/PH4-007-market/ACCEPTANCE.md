# PH4-007 Acceptance

- [x] MarketTradeId is typed and stable.
- [x] Market listings validate positive reference price, target stock, depth, spread, and demand window.
- [x] Quote derivation is read-only.
- [x] Scarcity changes price in the expected direction.
- [x] Recent unmet demand affects price only inside the configured window.
- [x] Low-price goods retain a non-zero bid/ask spread at fixed-point precision.
- [x] Trade execution uses deterministic depth/slippage.
- [x] Buy and sell settle goods and money atomically through EconomicTransaction.
- [x] Insufficient cash/inventory rejects without partial state.
- [x] Quote calculation does not create synthetic trades.
- [x] Trade history records actual executions with actual average price/value.
- [x] Replay and snapshot preserve market state/trades.
- [x] Snapshot schema advances for persisted market state.
- [x] Existing production/ScenarioPack/ledger tests remain green.
- [x] Full GitHub CI passes.

## Evidence

GitHub Actions run 37110636610 passed:
- Rust;
- reference-model;
- DuckDB analytics;
- Postgres schema.

The Rust gate also validates the v2 tiny ScenarioPack after the full workspace test suite.
