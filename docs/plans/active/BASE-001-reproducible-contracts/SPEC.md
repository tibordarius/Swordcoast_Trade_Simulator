# Feature Specification

## Task ID
BASE-001-reproducible-contracts

## Objective
Establish a reproducible build baseline and retain reviewed Contracts 0.1 and parallel-workflow design as repository proposals.

## Why it matters
Existing format drift and unpinned build context hinder reliable combined-head checks; research contracts need reviewable repository references and explicit implementation boundaries.

## User/system outcome
A pinned Rust toolchain, retained Cargo.lock, reproducibility manifest and strict check gate accompany proposed contracts, observable cases and dependency-preserving next jobs.

## Scope
Formatting normalization; Rust/Cargo dependency pinning; build manifest; strict CI/xtask checks; docs/execution proposals and this template-based task bundle.

## Non-goals
Implementing durable admission, branches, logistics, compaction or household economics; accepting a new ADR; changing economic behavior or rewriting v1; publishing externally.

## Domain/module
Repository build harness and execution design documentation.

## Dependencies
Pinned base a6ed57fc45e2e28b8c13708eb52d625ad4879ece; root AGENTS.md; ARCHITECTURE.md; accepted ADR-0001..0003; existing templates. Follow-on implementation requires its declared roadmap gates.

## Functional requirements
Normalize formatting separately from behavior changes. Pin compiler and dependency identity, retain a manifest identifying pack/build inputs, and keep CI warnings fatal. Preserve Contracts 0.1 semantics as Proposed/reviewed design and distinguish existing baseline cases from NOT IMPLEMENTED proposals. Preserve R-series dependencies and coordinator-owned shared interfaces.

## Invariants affected
No intentional runtime changes. Reducer-only authority, explicit ledger postings, integral authoritative economics, actor-local knowledge and immutable ScenarioPack boundary remain required.

## Data / migration implications
No runtime schema migration. Future format/codec changes require accepted design and versioned migration before implementation.

## Performance implications
No hot-path behavior change or performance claim; benchmarks are not a completion gate for formatting/documentation.

## References
../../../execution/CONTRACT.md; ../../../execution/ACCEPTANCE.csv; ../../../execution/NEXT.csv; ../../../execution/ROADMAP.csv; ../../../../AGENTS.md; ../../../../ARCHITECTURE.md; ../../../decisions/.
