# PH4-001 Plan

1. Add repo-local agent/architecture documentation.
2. Add restart-safe task templates and ADRs.
3. Create `sim-kernel-v2` crate.
4. Implement deterministic primitive types and RNG/hash utilities.
5. Add focused unit tests.
6. Add a minimal `xtask` command harness.
7. Register both crates in the existing Rust workspace.
8. Let existing CI exercise the workspace.

Compatibility:
- do not edit v1 source behavior;
- only workspace metadata is shared.
