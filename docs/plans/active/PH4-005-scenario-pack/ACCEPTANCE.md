# PH4-005 Acceptance

- [ ] scenario-pack-v2 is a standalone workspace crate.
- [ ] JSON pack loader parses the tiny fixture.
- [ ] unsupported schema versions are rejected.
- [ ] duplicate IDs are rejected.
- [ ] dangling unit/market/account/commodity references are rejected.
- [ ] zero/invalid unit conversion is rejected.
- [ ] invalid routes are rejected.
- [ ] reserved initialization IDs are rejected.
- [ ] opening balances compile into balanced reducer transactions.
- [ ] replaying compiled initialization commands is deterministic.
- [ ] tiny fixture creates 3 markets and 5 commodities.
- [ ] scenario-pack-validate CLI works.
- [ ] cargo xtask pack-validate works.
- [ ] full GitHub CI passes.
