#!/usr/bin/env python3
import json
from dataclasses import dataclass
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]

@dataclass(frozen=True)
class Opportunity:
    id:str
    roi_bps:int
    profit_mcp:int
    distance_km:int
    info_age_ticks:int
    bulk_score:int          # 0..10000
    value_density_score:int # 0..10000
    remote_score:int        # 0..10000
    controlled_route:bool
    risk_bps:int


def score(profile,o):
    p=profile['parameters']
    if o.roi_bps < p['min_roi_bps']: return None
    if o.info_age_ticks > p['max_information_age_ticks']: return None
    if o.distance_km > p['max_route_distance_km']: return None
    # Risk acceptance gate: opportunities more dangerous than profile tolerance are unseen/unacceptable.
    if o.risk_bps > p['risk_tolerance_bps']: return None
    s=o.roi_bps
    s += o.bulk_score*p['bulk_weight']//10000
    s += o.value_density_score*p['value_density_weight']//10000
    # distance_weight is calibrated per 1000 km.
    s += o.distance_km*p['distance_weight']//1000
    s += o.remote_score*p['remote_market_bonus']//10000
    if o.controlled_route: s += p['controlled_route_bonus']
    # Stale information still hurts within the allowed window. Better information quality reduces penalty.
    stale_penalty=o.info_age_ticks*(10000-p['information_quality_bps'])//10000
    s -= stale_penalty
    return s


def choose(profile,ops):
    scored=[(score(profile,o),o) for o in ops]
    scored=[x for x in scored if x[0] is not None]
    return max(scored,key=lambda x:(x[0],x[1].profit_mcp))[1] if scored else None


def main():
    data=json.loads((ROOT/'seed'/'merchant_profiles_v0.json').read_text())
    profiles={p['id']:p for p in data['profiles']}
    ops=[
      Opportunity('BULK-TIMBER-WD',900,180_000_000,720,120,9500,1200,1000,True,2200),
      Opportunity('REMOTE-SILK-EVERMEET',1250,155_000_000,4100,900,800,9800,9600,False,5400),
      Opportunity('SPICE-CAL-WD',1100,95_000_000,1700,350,1800,9000,4500,False,3000),
      Opportunity('LOCAL-GRAIN-BG-WD',1050,70_000_000,880,80,8200,900,500,False,1300)
    ]
    independent=choose(profiles['PROFILE-INDEPENDENT'],ops)
    otc=choose(profiles['PROFILE-OTC-BULK'],ops)
    dtc=choose(profiles['PROFILE-DTC-LONGRANGE'],ops)
    assert independent.id=='LOCAL-GRAIN-BG-WD', independent
    assert otc.id=='BULK-TIMBER-WD', otc
    assert dtc.id=='REMOTE-SILK-EVERMEET', dtc
    # DTC can still consider stale/distant intelligence that OTC rejects by range.
    assert score(profiles['PROFILE-OTC-BULK'],ops[1]) is None
    assert score(profiles['PROFILE-DTC-LONGRANGE'],ops[1]) is not None
    # Strategy engine contains no faction-name branches; binding is data only.
    print('SPRINT17_FACTION_STRATEGIES: PASS')
    print('independent_choice',independent.id)
    print('otc_profile_choice',otc.id)
    print('dtc_profile_choice',dtc.id)
    for pid in profiles:
      print(pid,[(o.id,score(profiles[pid],o)) for o in ops])

if __name__=='__main__': main()
