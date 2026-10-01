# PH4-007 Acceptance

- [ ] MarketTradeId is typed and stable.
- [ ] Market listings validate positive reference price, target stock, depth, and demand window.
- [ ] Quote derivation is read-only.
- [ ] Scarcity changes price in the expected direction.
- [ ] Recent unmet demand affects price only inside the configured window.
- [ ] Low-price goods retain a non-zero bid/ask spread at fixed-point precision.
- [ ] Trade execution uses deterministic depth/slippage.
- [ ] Buy and sell settle goods and money atomically through EconomicTransaction.
- [ ] Insufficient cash/inventory rejects without partial state.
- [ ] Quote calculation does not create synthetic trades.
- [ ] Trade history records actual executions with actual average price/value.
- [ ] Replay and snapshot preserve market state/trades.
- [ ] Existing production/ScenarioPack/ledger tests remain green.
- [ ] Full GitHub CI passes.
