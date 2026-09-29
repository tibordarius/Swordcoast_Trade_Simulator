# Sprint 6.5 — Native Build Gate

## Status

**Prepared, awaiting first GitHub run.**

The connected GitHub integration can write files and manage existing repositories but cannot create a new repository. The repository-ready project now includes the exact CI configuration needed once a private repository is created.

## Added

- `code/rust-toolchain.toml`
- `code/.github/workflows/ci.yml`
- `code/.gitignore`
- `code/compose.yaml`
- `code/.env.example`

## CI gates

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace --all-targets`
4. seed data validation
5. complete Python reference suite
6. PostgreSQL/PostGIS migration application
7. schema assertions against `postgis/postgis:18-3.6`

## Environment choice

The development database is pinned to PostgreSQL 18 + PostGIS 3.6. The upstream docker-postgis project lists `postgis/postgis:18-3.6` as a recommended current image and PostgreSQL 18 uses `/var/lib/postgresql` as its container volume path.

## Hard gate

Sprint 7 may be designed and reference-tested, but native Rust/database persistence should not be called production-ready until this workflow is green in an actual repository.
