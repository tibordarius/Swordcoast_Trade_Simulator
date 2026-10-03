# Acceptance Criteria

## Task ID
BASE-001-reproducible-contracts

- [x] Scoped repository/build behavior is implemented and reviewed by the integrator.
- [x] Focused checks pass.
- [x] Local workspace checks pass on the combined source; PostgreSQL returns early locally.
- [ ] Remote final-head PostgreSQL/integration gates pass (CI evidence required).
- [x] Relevant regression/determinism checks pass; no economic behavior change is introduced.
- [x] No architecture-boundary violations.
- [x] Documentation is updated for proposed contract changes.
- [x] Benchmark and UI evidence are not applicable: no hot-path or UI change is intended.

## Feature-specific criteria

- [x] Formatting-only changes are separated and inspected for semantic preservation.
- [x] Rust version and dependency lock are retained and used by the strict check gate.
- [x] Build manifest identifies compiler, lock and fixture/input identities reproducibly.
- [x] Strict CI is configured to reject drift/warnings; local results are recorded, remote CI still required.
- [x] Contracts 0.1 retains reviewed semantics with repository-relative references and explicit Proposed status.
- [x] Baseline case labels do not claim this task executed tests; proposed cases remain NOT IMPLEMENTED.
- [x] Bounded follow-on jobs retain dependency gates and distinguish research spikes from gated implementation.
- [x] ROADMAP.csv preserves the complete original R-series dependency DAG (integrator-owned).
- [x] Five template-based task documents exist; handoff remains REVIEW with local checks recorded and remote CI/review pending.
- [x] Independent combined-diff review and exact test/environment evidence are recorded before completion.
