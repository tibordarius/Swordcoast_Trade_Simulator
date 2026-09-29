#!/usr/bin/env python3
"""Dependency-free semantic oracle for Sprint 15 research queries.

This does not claim to test Parquet. It validates the analytical definitions that
native DuckDB must reproduce in CI.
"""
from collections import defaultdict
from dataclasses import dataclass

@dataclass(frozen=True)
class Tick:
    tick:int; market:str; commodity:str; price:int; volume:int


def fixture():
    rows=[]
    # 48 observations: two markets, one commodity, 24 ticks each.
    for t in range(24):
        wd=2000 + t*4 + (20 if t%5==0 else 0)
        bg=1800 + t*3 + (10 if t%7==0 else 0)
        rows.append(Tick(t,'WDEX','GRAIN',wd,1000+t*10))
        rows.append(Tick(t,'BGEX','GRAIN',bg,800+t*7))
    return rows


def spreads(rows):
    by=defaultdict(dict)
    for r in rows: by[r.tick][r.market]=r.price
    return [(t,v['WDEX']-v['BGEX']) for t,v in sorted(by.items()) if 'WDEX' in v and 'BGEX' in v]


def volumes(rows):
    out=defaultdict(int)
    for r in rows: out[(r.market,r.commodity)]+=r.volume
    return dict(out)


def volatility_bps(rows):
    grouped=defaultdict(list)
    for r in rows: grouped[(r.market,r.commodity)].append(r)
    ans={}
    for k,rr in grouped.items():
        rr.sort(key=lambda r:r.tick)
        moves=[]
        for a,b in zip(rr,rr[1:]):
            moves.append(abs(b.price-a.price)*10000//a.price)
        ans[k]=(sum(moves)/len(moves),max(moves))
    return ans


def route_mtm_roi_bps(purchase, freight, destination_mark):
    invested=purchase+freight
    return (destination_mark-invested)*10000//invested


def main():
    rows=fixture(); s=spreads(rows); v=volumes(rows); vol=volatility_bps(rows)
    assert len(s)==24
    assert s[0]==(0,210)
    assert s[-1]==(23,223)
    assert v[('WDEX','GRAIN')]==26760
    assert v[('BGEX','GRAIN')]==21132
    assert round(vol[('WDEX','GRAIN')][0],6)==48.652174
    assert vol[('WDEX','GRAIN')][1]==119
    roi=route_mtm_roi_bps(165_500_000,31_500_000,337_800_000)
    assert roi==7147
    print('SPRINT15_ANALYTICS_REFERENCE: PASS')
    print('spread_first_mcp',s[0][1])
    print('spread_last_mcp',s[-1][1])
    print('wd_volume_milli',v[('WDEX','GRAIN')])
    print('wd_mean_abs_move_bps',f'{vol[("WDEX","GRAIN")][0]:.6f}')
    print('arrival_mtm_roi_bps',roi)

if __name__=='__main__': main()
