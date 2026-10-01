# PH4-003 Plan

1. Add inventory/money account and transaction ID types.
2. Add ledger/account module.
3. Add EconomicTransaction and posting types.
4. Extend WorldState with account balances, ledgers, and applied transaction IDs.
5. Extend Command with account-open and ApplyTransaction variants.
6. Add validation before mutation.
7. Commit atomically through WorldReducer.
8. Add conservation, duplicate, negative-balance, atomicity, replay, and snapshot tests.

Compatibility:
- PH4-002 replay/snapshot behavior must remain green;
- v1 remains untouched.
