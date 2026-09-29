# [[Sprint 21 — MVP Release Preparation]]

## Status

**Reference release gate: PASS. Native release gate: BLOCKED by environment/tooling.**

## One-command gates

From the project root:

```bash
make reference
make preflight
python tools/release_check.py --reference-only
```

On a fully provisioned development machine:

```bash
python tools/release_check.py
```

The strict native gate requires:

- Node;
- Rust `cargo` + `rustc`;
- Docker;
- PostgreSQL client (`psql`);
- DuckDB Python package;
- green Rust workspace tests;
- green native DuckDB/Parquet gate;
- green reference suite and migration lint.

## Static prototype

The dependency-free WDEX/map/DM prototypes can be served with:

```bash
make static
```

then inspected under the appropriate `wdex/`, `map/` and `dm/` paths.

This is a developer inspection path, not the final bundled React application.
