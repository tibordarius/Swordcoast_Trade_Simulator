# Implementation Plan

## Task ID
BASE-001-reproducible-contracts

## Current state
Base a6ed57fc45e2e28b8c13708eb52d625ad4879ece retains accepted v2 architecture and baseline foundations. Reviewed execution research is a proposal, not implemented functionality. Build/check evidence is pending integrator completion.

## Proposed contract changes
Retain reviewed Contracts 0.1 unchanged in substantive semantics with repository-relative references. Add proposed parallel-workflow controls and preserve acceptance/roadmap statuses. No accepted ADR changes.

## Files/modules expected to change
Integrator owns formatting, toolchain/Cargo.lock, build manifest and strict CI/xtask configuration. Documentation worker exclusively owns docs/execution/{CONTRACT.md,ACCEPTANCE.csv,NEXT.csv} and this five-file task bundle. Integrator owns docs/execution/ROADMAP.csv. Exact final code/config scope is recorded by the integrator in HANDOFF.md.

## Sequence
1. Inspect repository authority, templates and reviewed inputs at the pinned base.
2. Normalize formatting separately; retain compiler/dependency/build identity and strict checks.
3. Adopt proposal documents and preserve the full roadmap DAG alongside bounded follow-on jobs.
4. Review the concrete combined diff and run narrow checks then required workspace gates.
5. Record command/environment/results and unresolved acceptance in handoff; integrate only after review.

## Test plan
From code/: cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; cargo xtask check; cargo xtask test-v2; cargo xtask test-tiny; cargo xtask pack-validate crates/scenario-pack-v2/tests/fixtures/tiny_sword_coast.json. Record exact results rather than treating test-tiny as century validation. Verify document references, CSV structure and unchanged dependency DAG; classify unavailable infrastructure checks explicitly.

## Migration / compatibility plan
Format-6 remains the compatibility reference. Preserve existing economic semantics, source behavior and accepted ADRs. No proposed case becomes implemented merely through document adoption.

## Performance risks
Build reproducibility does not imply archive/recompute retention is implemented. No runtime benchmark claim.

## Rollback strategy
Integrator may revert scoped formatting/config/documentation patches independently; no live state migration is involved.

## Open questions
Final combined-head and infrastructure checks pending. Partial PH4-009 must be assessed before its later implementation patch, not completed by this task.
