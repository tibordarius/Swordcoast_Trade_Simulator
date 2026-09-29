# Sword Coast Economic Simulator — Code Workspace

## Sprint 1 target

This workspace begins with a dependency-free `sim-core` so the determinism kernel can compile without network access once a Rust toolchain is present.

### Intended checks

```bash
cargo test --workspace
cargo run -p sim-cli -- 12345 10000
```

Expected reference result for seed `12345`, 10,000 ticks:

```text
production_signal=-19
state_hash=ea50aa8c6fbd16cb
```

The expected value is generated independently by `../reference/sprint1_reference.py` using the same documented SplitMix64 and FNV-1a algorithms.

## Current environment limitation

The ChatGPT execution container used on 2026-09-29 does not include `rustc` or `cargo` and cannot resolve `sh.rustup.rs`, so these Rust tests have not been compiled here. The Python reference oracle has been executed successfully.
