# PH4-003 Atomic Economic Transactions

## Objective
Prove one authoritative, atomic transaction path for goods and money.

## Scope
- inventory and money account IDs;
- account kinds;
- zero-balance account opening through commands;
- EconomicTransaction;
- inventory and money postings;
- transaction validation;
- reducer commit;
- authoritative ledgers;
- conservation and atomicity tests.

## Non-goals
- markets;
- production recipes;
- shipment logistics;
- credit/loans;
- database persistence;
- UI.

## Invariants
- every transaction is balanced;
- holding accounts may not go negative;
- source/external accounts may absorb explicit sources/sinks;
- duplicate transaction IDs are rejected;
- validation failure commits nothing;
- successful command increments revision once;
- all ledger changes pass through WorldReducer.
