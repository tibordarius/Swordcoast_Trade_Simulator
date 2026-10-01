# Agent Guide

## Project
Sword Coast / Toril economic simulator. Phase 4 introduces a new `sim-kernel-v2` alongside the frozen v1 reference implementation.

## Non-negotiable architecture
- Only the v2 reducer may commit authoritative world state.
- Goods and money move through explicit transactions/ledger postings.
- No authoritative economic `f64` in v2 value types.
- Actors receive a knowledge view, not global market truth.
- TorilGIS integrates through versioned ScenarioPack data. The kernel does not crawl source files at runtime.
- UI/API code may issue commands and queries, but may not mutate kernel internals directly.
- v1 remains a regression/reference implementation. Do not rewrite it while building v2.

## Start here
- `ARCHITECTURE.md`
- `docs/decisions/`
- `docs/plans/active/`
- `docs/research/INDEX.md`

## Verification
From `code/`:
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets`

Run the narrowest relevant test first, then the broader workspace gate before merge.

## Task workflow
A substantial task should have:
- SPEC.md
- PLAN.md
- ACCEPTANCE.md
- HANDOFF.md
- CHECKPOINT.md

Use the templates in `docs/templates/`.

## Completion
Do not report a task complete unless:
- acceptance criteria are satisfied;
- relevant tests pass;
- architecture boundaries remain intact;
- documentation is updated for contract changes;
- the current handoff/result state is recorded.

## Scope discipline
Keep changes small and reviewable. Do not use a task as permission to redesign unrelated modules.
