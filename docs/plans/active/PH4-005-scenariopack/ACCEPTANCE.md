# PH4-005 Acceptance

- [ ] ScenarioPack schema version is explicit and validated.
- [ ] Commodity/place/route/account IDs must be unique.
- [ ] Route endpoints must exist.
- [ ] Opening balances reference declared accounts/commodities.
- [ ] Active commodities require positive mass and volume dimensions.
- [ ] Holding opening balances cannot be negative.
- [ ] Source/campaign/derived/generated status is explicit.
- [ ] Loader initializes WorldState only through WorldReducer commands.
- [ ] Opening goods and money are balanced against explicit seed source/external accounts.
- [ ] JSON roundtrip preserves the pack.
- [ ] Re-loading the same pack yields the same registry and world state hash.
- [ ] Tiny-world fixture loads successfully.
- [ ] Full GitHub CI passes.
