# Handoff

## Task
BASE-001-reproducible-contracts

## Goal
Strict Rust baseline, retained build inputs and repository proposal/workflow adoption.

## Status
REVIEW. Local scoped checks and independent reviews passed. Draft PR review and final remote CI/merge remain separate.

## Branch / worktree
chore/toril-baseline-contracts-2026-10-03, isolated implementation worktree. Input main a6ed57fc45e2e28b8c13708eb52d625ad4879ece.

## Last known green commit
Code/build commit 2945e3a passed the local strict gate with proposed documentation present. The formatting-only commit is fa22d2c. Final published-head evidence is supplied by CI and its build-manifest artifact; do not infer merge status from this document.

## Completed
47 Rust files normalized by rustfmt in a separate commit; Rust1.99.0 and Cargo.lock retained; xtask bootstrap/subprocesses locked; CI rejects formatting drift; verified build-manifest recorder added. Contracts0.1, 30 cases, original37DAG, 11 bounded jobs, three local Sol roles and persistent parallel workflow added. Existing accepted ADRs and Proposed contract status preserved.

## Tests last run
From code/, Rust1.99.0 and retained lock: cargo xtask test-tiny PASS (57 kernel +22 pack tests); cargo xtask check PASS (strictfmt, clippy -Dwarnings, workspace112 reported passes); cargo xtask pack-validate tinyfixture PASS. Local PostgreSQL integration did not execute because no service/URL; its test returns early. Prior baseline reference/DuckDB checks passed; CI reruns its full configured jobs. See RESULT.md for scope/evidence.

## Current failure / blocker
No local strict Rust failure. PostgreSQL was not exercised locally. Proposed execution interfaces, household economics and partial PH4-009 logistics are outside this maintenance patch. Logistics branch remains unmerged/incomplete.

## Decisions made
Contracts0.1 remains Proposed; existing ADRs govern executable code. Build manifest records inputs/integrity, not an executable archive or identical-binary proof. Hosted Work does not consume local TOML. Up to3 workers is an evaluated starting policy, not an optimum.

## Important files
../../../execution/CONTRACT.md; ../../../execution/{ACCEPTANCE,ROADMAP,NEXT}.csv; ../../../agents/WORKFLOW.md; rootAGENTS; .codex roleconfig; code/reproducibility; tools/write_build_manifest.py; rootCI; code/Cargo.lock; RESULT.md.

## Next action
Review draft PR and exact-head CI. After baseline/adoption gate, assign independent continuity/data, information-policy, household budget and protected production work per NEXT.csv; assess/reuse the incomplete PH4-009 branch before logistics implementation. Original R-series gates remain mandatory.

## Remaining acceptance criteria
Remote exact-head integration evidence and human merge/review; future proposed runtime acceptance is separate and NOT IMPLEMENTED.

## Do not redo
Do not repeat formatting normalization, recreate PH4 foundations or duplicate PH4-009. Do not count v1API/PG as v2durability, or test-tiny as a century run. Do not mark task DONE just because a handoff says REVIEW.
