#!/usr/bin/env python3
import importlib.util,json,shutil,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def cmd(name):
    p=shutil.which(name); return {"available":bool(p),"path":p}

def version(argv):
    try:return subprocess.check_output(argv,text=True,stderr=subprocess.STDOUT,timeout=5).strip().splitlines()[0]
    except Exception:return None

def main():
    checks={
      "python":{"available":True,"version":sys.version.split()[0]},
      "node":cmd("node"),"cargo":cmd("cargo"),"rustc":cmd("rustc"),"docker":cmd("docker"),"psql":cmd("psql"),
      "duckdb_python":{"available":importlib.util.find_spec("duckdb") is not None}
    }
    if checks['node']['available']:checks['node']['version']=version(['node','--version'])
    if checks['cargo']['available']:checks['cargo']['version']=version(['cargo','--version'])
    if checks['rustc']['available']:checks['rustc']['version']=version(['rustc','--version'])
    if checks['docker']['available']:checks['docker']['version']=version(['docker','--version'])
    if checks['psql']['available']:checks['psql']['version']=version(['psql','--version'])
    release_required=['node','cargo','rustc','docker','psql','duckdb_python']
    blockers=[k for k in release_required if not checks[k]['available']]
    out={"project":"Sword Coast Economic Simulator","reference_suite":"tools/run_reference_suite.py","checks":checks,"native_release_blockers":blockers,"native_ready":not blockers}
    if '--json' in sys.argv:print(json.dumps(out,indent=2))
    else:
      print('WDEX PREFLIGHT')
      for k,v in checks.items():print(f"{k:16} {'OK' if v['available'] else 'MISSING'} {v.get('version') or v.get('path') or ''}")
      print('native_ready',out['native_ready'])
      if blockers:print('blockers',', '.join(blockers))
if __name__=='__main__':main()
