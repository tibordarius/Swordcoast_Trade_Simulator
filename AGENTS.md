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
- `cargo xtask check`
- `cargo xtask test-v2`
- `cargo xtask test-tiny`
- `cargo xtask pack-validate crates/scenario-pack-v2/tests/fixtures/tiny_sword_coast.json`

The underlying workspace gates remain:
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

## Parallel work
Use up to three subagents for independent bounded tasks when useful. Follow `docs/agents/WORKFLOW.md` and preserve the five existing task documents. The coordinator owns dependency/ownership assignment, shared contracts, serial integration and publication. Workers get pinned inputs and exclusive paths; reviewers examine concrete results independently. Do not recursively delegate unless the coordinator explicitly revises the allocation. Proposed execution decisions live in `docs/execution/CONTRACT.md`; existing accepted ADRs remain authoritative until a reviewed implementation adopts a versioned change.

## Build input records
Use the pinned Rust toolchain and committed `code/Cargo.lock`. `cargo xtask` locks dependency resolution. `code/reproducibility/README.md` explains verified build-input manifests and their limits. A recorded hash is not an archived executable or proof of a complete campaign runtime.
