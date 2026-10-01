# PH4-003 Acceptance

- [ ] Accounts can only be opened through reducer commands.
- [ ] Inventory and money postings are balanced.
- [ ] Holding accounts cannot become negative.
- [ ] Explicit source/external accounts may carry balancing negative totals.
- [ ] Duplicate transaction IDs are rejected.
- [ ] Invalid mixed transaction changes neither goods nor money.
- [ ] Successful transaction writes authoritative ledger entries.
- [ ] Replay with transactions is deterministic.
- [ ] Snapshot roundtrip preserves transaction/ledger state.
- [ ] cargo test -p sim-kernel-v2 passes.
- [ ] full GitHub CI passes.
