# Sprint 9 — WDEX Shell

## Status

**Functional dependency-free browser shell implemented against the v1 snapshot/delta contract.**

## Delivered

`code/web/wdex/`

- `index.html`
- `styles.css`
- `app.js`
- `market.mjs`
- `mock-snapshot.json`
- `market.test.mjs`

## Current UI

The shell includes:

- WDEX exchange header and simulation sequence/tick status;
- sortable-style market table foundation with six reference commodities;
- exact bid/ask display from milli-copper values;
- daily percentage movement;
- local stock and days-of-cover display;
- selected commodity detail panel;
- buy/sell order-preview ticket;
- slippage/market-impact preview;
- causal-market-intelligence panel placeholder.

## Why no framework yet

The goal of this sprint is to validate data semantics and trading interactions before adding React, charts and WebSocket libraries. The pure market functions are independently testable in Node and use `BigInt` at the protocol boundary.

## Tests

`node --test code/web/wdex/market.test.mjs`

Covers:

- values above JavaScript's safe integer range;
- exact percentage movement;
- days-of-cover calculation;
- live-delta gap rejection;
- market-order slippage preview.

## Next

Sprint 10 connects the ticket to command semantics and introduces player wallet, warehouse inventory, physical cargo transfer and market/limit-order lifecycle.
