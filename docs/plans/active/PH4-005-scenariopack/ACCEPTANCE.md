# PH4-005 Acceptance

- [x] ScenarioPack schema version is explicit and validated.
- [x] Commodity/place/route/account IDs must be unique.
- [x] Route endpoints must exist.
- [x] Opening balances reference declared accounts/commodities.
- [x] Active commodities require positive mass and volume dimensions.
- [x] Holding opening balances cannot be negative.
- [x] Source/campaign/derived/generated status is explicit.
- [x] Initialization creates WorldState only through WorldReducer/replay commands.
- [x] Opening goods and money are balanced against explicit seed source/external accounts.
- [x] JSON roundtrip preserves the pack.
- [x] Re-loading equivalent pack data yields the same registry and world state hash.
- [x] Tiny-world fixture loads successfully.
- [x] Full GitHub CI passes.

## Evidence

GitHub Actions run 36886057579 passed:
- Rust;
- reference-model;
- DuckDB analytics;
- Postgres schema.
