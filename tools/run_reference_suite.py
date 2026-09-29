#!/usr/bin/env python3
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
REF = ROOT / "reference"
WEB = ROOT / "code" / "web" / "wdex"
MAP = ROOT / "code" / "web" / "map"
ANALYTICS = ROOT / "code" / "web" / "analytics"


def run(label, argv, cwd=ROOT):
    print(f"\n=== {label} ===")
    subprocess.run(argv, cwd=cwd, check=True)


def main():
    run("seed validation", [sys.executable, str(ROOT / "tools" / "validate_seed.py")])
    for script in [
        "sprint1_reference.py",
        "sprint2_grain.py",
        "sprint3_pricing.py",
        "sprint4_logistics.py",
        "sprint5_merchant.py",
        "sprint6_validation.py",
        "sprint7_persistence.py",
    ]:
        run(script, [sys.executable, str(REF / script)], REF)

    run("migration lint", [sys.executable, str(ROOT / "tools" / "lint_migrations.py")])

    node = shutil.which("node")
    if not node:
        raise SystemExit("node is required for Sprint 8–13 reference tests")
    run("Sprint 8 live protocol", [node, str(REF / "sprint8_live_protocol.mjs")], REF)
    run("Sprint 9 WDEX shell", [node, "--test", str(WEB / "market.test.mjs")], WEB)
    run("Sprint 10 trading", [node, "--test", str(WEB / "trading.test.mjs")], WEB)
    run("Sprint 11 multi-market", [node, "--test", str(WEB / "network.test.mjs")], WEB)
    run("Sprint 12 map calibration", [node, "--test", str(MAP / "calibration.test.mjs")], MAP)
    run("Sprint 12b Toril GCS", [node, str(MAP / "toril-gcs.test.mjs")], MAP)
    run("Sprint 13 route geometry build", [node, str(ROOT / "tools" / "build_route_geometries.mjs")], ROOT)
    run("Sprint 13 live logistics", [node, str(MAP / "live-logistics.test.mjs")], MAP)
    run("Sprint 13b pathways", [node, str(REF / "sprint13b_pathways.test.mjs")], REF)
    run("Sprint 14 history unit tests", [node, "--test", str(WEB / "history.test.mjs")], WEB)
    run("Sprint 14 history benchmark", [node, str(REF / "sprint14_history_benchmark.mjs")], REF)
    run("Sprint 15 analytics semantics", [sys.executable, str(ROOT / "tools" / "sprint15_analytics_reference.py")], ROOT)
    run("Sprint 15 analytics query contract", [node, "--test", str(ANALYTICS / "query-contract.test.mjs")], ANALYTICS)
    run("Sprint 16 production chain", [sys.executable, str(REF / "sprint16_production_chain.py")], REF)
    run("Sprint 17 faction strategies", [sys.executable, str(REF / "sprint17_faction_strategies.py")], REF)
    run("Sprint 18 DM control", [node, "--test", str(ROOT / "code" / "web" / "dm" / "control.test.mjs")], ROOT / "code" / "web" / "dm")
    run("Sprint 19 scenarios", [sys.executable, str(REF / "sprint19_scenarios.py")], REF)
    run("Sprint 20 trading fuzz", [node, "--test", str(WEB / "hardening.test.mjs")], WEB)
    run("Sprint 20 recovery", [sys.executable, str(REF / "sprint20_recovery.py")], REF)

    print("\nREFERENCE SUITE 0–20: PASS")

if __name__ == "__main__":
    main()
