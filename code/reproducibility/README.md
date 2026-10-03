# Recorded Rust build inputs

Rust is pinned in ../rust-toolchain.toml and resolved dependencies are committed in ../Cargo.lock. Cargo xtask locks its bootstrap and each dependency-resolving subprocess. Active root CI uses the same toolchain, checks formatting without rewriting sources and uses locked dependency gates.

inputs.json records exact lock and tiny-fixture digests. After reviewing an intentional dependency/fixture change, update those digests in the same patch. They are artifact identities, not lore approval. Snapshot format 6 and ScenarioPack schema 1 describe this baseline.

From repository root with the pinned Rust toolchain installed:

```sh
python tools/write_build_manifest.py --output /tmp/toril-build.json
```

To record an existing validator binary, also pass --artifact code/target/debug/scenario-pack-validate. CI writes a manifest after validation and uploads it as rust-build-manifest. The record contains actual source commit, tracked-dirty status, compiler/cargo identity, host platform, verified input digests and optional binary digest. Commit changes before producing a release record; a dirty worktree cannot be reproduced from its commit alone.

This pins Rust inputs and records integrity. It does not archive an executable, pin every CI runner/action/container/Python dependency, or prove identical binaries across hosts. Durable simulation recovery additionally needs retained engine artifacts, codecs, rule/continuity/RNG versions and the full execution context in the proposed execution contract.

The old code/README environment note describes September 29 and should not be interpreted as the present environment. Current build commands and evidence are recorded in the active BASE-001 task.
