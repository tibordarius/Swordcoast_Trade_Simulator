#!/usr/bin/env python3
"""Record verified build inputs and optional artifact integrity, not a binary archive."""

import argparse
import hashlib
import json
import platform
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CODE = ROOT / "code"


def run(*args, cwd=ROOT):
    return subprocess.check_output(args, cwd=cwd, text=True).strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, help="Write JSON here; default is stdout.")
    parser.add_argument("--artifact", type=Path, help="Digest an existing artifact, relative to cwd.")
    args = parser.parse_args()

    inputs = json.loads((CODE / "reproducibility/inputs.json").read_text())
    lock = CODE / "Cargo.lock"
    fixture = CODE / inputs["scenario_fixture"]["path"]
    for label, actual, expected in [
        ("Cargo.lock", digest(lock), inputs["cargo_lock_sha256"]),
        ("ScenarioPack fixture", digest(fixture), inputs["scenario_fixture"]["sha256"]),
    ]:
        if actual != expected:
            raise SystemExit(f"{label} differs from the recorded inputs; review and update inputs.json.")

    fixture_manifest = json.loads(fixture.read_text())["manifest"]
    for key in ("schema_version", "revision"):
        if fixture_manifest[key] != inputs["scenario_fixture"][key]:
            raise SystemExit(f"ScenarioPack {key} differs from recorded inputs.")
    version_constants = [
        (CODE / "crates/sim-kernel-v2/src/snapshot.rs", "SNAPSHOT_FORMAT_VERSION", inputs["snapshot_format_version"]),
        (CODE / "crates/scenario-pack-v2/src/model.rs", "SCENARIO_PACK_SCHEMA_VERSION", inputs["scenario_fixture"]["schema_version"]),
    ]
    for source, constant, expected in version_constants:
        match = re.search(rf"pub const {constant}: u32 = (\d+);", source.read_text())
        if not match or int(match.group(1)) != expected:
            raise SystemExit(f"{constant} differs from recorded inputs.")

    compiler = run("rustc", "--version", "--verbose", cwd=CODE)
    if f"release: {inputs['rust_toolchain']}" not in compiler.splitlines():
        raise SystemExit("Compiler release differs from the pinned Rust toolchain.")
    run("cargo", "metadata", "--locked", "--no-deps", "--format-version=1", cwd=CODE)

    manifest = {
        "schema_version": 1,
        "source_commit": run("git", "rev-parse", "HEAD"),
        "tracked_worktree_dirty": bool(run("git", "status", "--porcelain", "--untracked-files=no")),
        "inputs": inputs,
        "rustc_verbose": compiler,
        "cargo_version": run("cargo", "--version", cwd=CODE),
        "platform": {"system": platform.system(), "machine": platform.machine()},
        "artifact": None,
        "scope": "Build-input and integrity record. Does not archive an executable or prove identical binaries.",
    }
    if args.artifact:
        manifest["artifact"] = {
            "name": args.artifact.name,
            "sha256": digest(args.artifact),
            "size_bytes": args.artifact.stat().st_size,
        }
    content = json.dumps(manifest, indent=2) + "\n"
    if args.output:
        args.output.write_text(content)
    else:
        print(content, end="")


if __name__ == "__main__":
    main()
