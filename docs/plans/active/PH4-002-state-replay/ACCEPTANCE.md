# PH4-002 Acceptance

- [ ] WorldState fields are not publicly mutable.
- [ ] AdvanceTo cannot regress time.
- [ ] Successful command increments revision once.
- [ ] Replay rejects non-monotonic command sequence.
- [ ] Same initial state + commands produces identical state hash.
- [ ] Snapshot encode/decode preserves state hash.
- [ ] Unsupported snapshot format is rejected.
- [ ] cargo test -p sim-kernel-v2 passes.
- [ ] full GitHub CI passes.
