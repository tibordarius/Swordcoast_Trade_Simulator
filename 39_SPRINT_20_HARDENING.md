# [[Sprint 20 — Hardening]]

## Status

**Reference hardening complete. Native platform gates remain outstanding.**

## Added gates

### Randomized trading invariants

2,000 deterministic pseudo-random market buys/sells are executed while asserting after every operation:

- market inventory never becomes negative;
- player cash never becomes negative;
- reserved cash never exceeds cash;
- warehouse quantities never become negative;
- reserved inventory never exceeds owned inventory.

Result: PASS.

### Snapshot + event-log recovery

A state is reconstructed from snapshot plus ordered event log and must reproduce the same SHA-256 state hash.

The test then modifies one event by one unit and verifies that the chained event hash detects tampering.

Result: PASS.

### Performance budgets

Current reference budgets include:

| Operation | Budget | Recent local result |
|---|---:|---:|
| 10,000 derived shipment positions | 100 ms | ~25 ms |
| 100,000 history rows → candles | 500 ms | ~107 ms |
| 100 scenario futures | 1,000 ms | gate defined |
| 2,000 ledger operations + invariants | 1,000 ms | ~8 ms test body |

Budgets are regression guardrails, not promises across all hardware.
