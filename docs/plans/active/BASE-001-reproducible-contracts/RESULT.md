# BASE-001 local result

Source input: main a6ed57fc45e2e28b8c13708eb52d625ad4879ece. Formatting commitfa22d2c; build commit2945e3a. Checks ran in the isolated implementation worktree with Rust1.99.0, locked dependencies and a coordinator-owned external target directory.

| Check | Result | Limit |
| --- | --- | --- |
| cargo xtask test-tiny | PASS,79 reported | Kernel57 plus pack22; overlaps workspace |
| cargo xtask check | PASS | Strictfmt, clippy, workspace112 reported |
| cargo xtask pack-validate tinyfixture | PASS | Generated fixture, not canon/date approval |
| TOML /30cases /37DAG /11jobs | PASS | Proposed job dependencies preserved |
| Formatting-only byte comparison | PASS | Reviewer rustfmt'd every baseline file;47exactmatches |
| Build manifest helper | PASS | Lock/fixture/compiler/codec verified; optional artifact integrity only |
| Independent docs/config review | PASS after path fix | Eight proposal sections preserved; local/Work distinction correct |
| PostgreSQL integration locally | NOT RUN | Test returns early without WDEX_TEST_DATABASE_URL |

No new economic transition behavior is introduced. Clippy/tests run using the normalized source; the only Rust tool behavior change is dependency locking in xtask. Root CI now verifies formatting instead of modifying source. Its service/migration gates remain intact and must establish remote DB evidence at the published head.

Manifest metadata records the actual checkout commit and dirty flag, so a release record must be generated from the committed checkout. Cargo.lock and exact Rust support repeatable inputs; mutable runner/action/container/Python components and absent executable archival remain limitations.

The contracts and role workflows are proposed/adopted repository documents, not implementation of request receipts, owned cargo, branches, shocks or century economics. Task status remains REVIEW until normal PR review/integration.
