#!/usr/bin/env python3
import argparse,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def run(argv,cwd=ROOT):
 print('+',' '.join(map(str,argv)));return subprocess.run(argv,cwd=cwd).returncode

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--reference-only',action='store_true');a=ap.parse_args()
 if run([sys.executable,ROOT/'tools'/'run_reference_suite.py']):return 1
 if run([sys.executable,ROOT/'tools'/'lint_migrations.py']):return 1
 if a.reference_only:
  print('REFERENCE RELEASE GATE: PASS');return 0
 # Native preflight is intentionally strict.
 p=subprocess.run([sys.executable,ROOT/'tools'/'preflight.py','--json'],capture_output=True,text=True)
 print(p.stdout,end='')
 import json
 state=json.loads(p.stdout)
 if not state['native_ready']:
  print('NATIVE RELEASE GATE: BLOCKED');return 2
 if run(['cargo','test','--workspace','--all-targets'],ROOT/'code'):return 1
 if run([sys.executable,ROOT/'tools'/'sprint15_duckdb_gate.py']):return 1
 print('NATIVE RELEASE GATE: PASS');return 0
if __name__=='__main__':raise SystemExit(main())
