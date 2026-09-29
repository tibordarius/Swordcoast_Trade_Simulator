# Sprint 10 — Trading, Warehouses & Physical Cargo

## Status

**Trading lifecycle implemented and reference-tested in the browser domain model. Persistence schema extended for warehouses and vessel cargo.**

## Delivered

### Trading domain

`code/web/wdex/trading.mjs`

Supports:

- exact wallet balances in milli-copper;
- reserved vs available cash;
- warehouse ownership positions;
- reserved vs available goods;
- market buy and sell execution;
- stale-market-state rejection using sequence watermarks;
- buy/sell limit-order reservation and fill logic;
- transfer of owned warehouse goods into physical vessel cargo;
- mass and volume capacity checks.

### Persistence

`code/migrations/0003_warehousing.sql`

Adds:

- warehouse;
- warehouse positions;
- vessel;
- vessel cargo;
- reserved cash on market orders.

## Tests

`node --test code/web/wdex/trading.test.mjs`

The suite verifies:

1. market purchase transfers title into warehouse stock and charges the wallet;
2. a stale quote cannot mutate state;
3. limit buys reserve funds and only execute when the executable average price clears the limit;
4. warehouse stock can be moved into vessel cargo without duplication;
5. market sales return goods to local market inventory and credit the wallet.

## Important distinction now represented

Owning a commodity in WDEX is not the same as physically carrying it.

`market purchase → warehouse title → cargo transfer → shipment`

This is the foundation for player speculation, storage, chartering, smuggling and physical trade voyages.
