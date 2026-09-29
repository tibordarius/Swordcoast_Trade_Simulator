#!/usr/bin/env python3
"""Version-pinned Toril GIS importer.

Downloads the exact GeoJSON files named in source-manifest.json and optionally clips
features to a broad bounding box. It never follows upstream main/latest implicitly.
"""
from __future__ import annotations
import argparse, hashlib, json, urllib.request
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/'code/data/toril/source-manifest.json'
OUT=ROOT/'code/data/toril/cache'

def bbox_coords(obj):
    xs=[];ys=[]
    def walk(v):
        if isinstance(v,list) and len(v)>=2 and all(isinstance(x,(int,float)) for x in v[:2]):
            xs.append(v[0]);ys.append(v[1]);return
        if isinstance(v,list):
            for x in v: walk(x)
    walk(obj)
    return None if not xs else (min(xs),min(ys),max(xs),max(ys))

def intersects(a,b):
    return not (a[2]<b[0] or a[0]>b[2] or a[3]<b[1] or a[1]>b[3])

def clipped(fc,bbox):
    keep=[]
    for f in fc.get('features',[]):
        geom=f.get('geometry') or {}
        bb=bbox_coords(geom.get('coordinates'))
        if bb and intersects(bb,bbox): keep.append(f)
    return {**fc,'features':keep}

def sha256(data:bytes): return hashlib.sha256(data).hexdigest()

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument('--offline',action='store_true',help='validate existing cache only')
    ap.add_argument('--no-clip',action='store_true')
    args=ap.parse_args()
    manifest=json.loads(MANIFEST.read_text())
    OUT.mkdir(parents=True,exist_ok=True)
    receipt={'source_commit':manifest['commit'],'export_batch':manifest['export_batch'],'files':[]}
    bbox=manifest['initial_clip']['bbox_fria']
    for layer in manifest['layers']:
        raw=OUT/layer['file']
        if not raw.exists():
            if args.offline: raise SystemExit(f'missing cached layer: {raw.name}')
            url=f"{manifest['base_raw_url']}/{layer['file']}"
            with urllib.request.urlopen(url,timeout=120) as r: data=r.read()
            raw.write_bytes(data)
        data=raw.read_bytes()
        fc=json.loads(data)
        record={'layer':layer['id'],'file':layer['file'],'sha256':sha256(data),'features_total':len(fc.get('features',[]))}
        if not args.no_clip:
            subset=clipped(fc,bbox)
            clipped_path=OUT/(raw.stem+'.sword-coast.geojson')
            clipped_path.write_text(json.dumps(subset,separators=(',',':')))
            record['features_sword_coast']=len(subset['features'])
            record['clip_file']=clipped_path.name
        receipt['files'].append(record)
    (OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt,indent=2))

if __name__=='__main__': main()
