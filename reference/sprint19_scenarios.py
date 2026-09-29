#!/usr/bin/env python3
import copy, hashlib, random, statistics
from dataclasses import dataclass

@dataclass
class State:
    tick:int
    wd_grain:int
    route_capacity_bps:int
    route_risk_bps:int
    price_mcp:int


def state_hash(s):
    raw=f'{s.tick}|{s.wd_grain}|{s.route_capacity_bps}|{s.route_risk_bps}|{s.price_mcp}'.encode()
    return hashlib.sha256(raw).hexdigest()


def derive_seed(world_seed, namespace, run_index=0):
    raw=f'{world_seed}|{namespace}|{run_index}'.encode()
    return int.from_bytes(hashlib.blake2b(raw,digest_size=8).digest(),'big')


def run_from_snapshot(snapshot,event=None,days=30,seed=1):
    s=copy.deepcopy(snapshot); rng=random.Random(seed)
    for day in range(days):
        if event and event['start_day']<=day<event['end_day']:
            s.route_capacity_bps=event.get('route_capacity_bps',s.route_capacity_bps)
            s.route_risk_bps=event.get('route_risk_bps',s.route_risk_bps)
        else:
            s.route_capacity_bps=10000; s.route_risk_bps=1200
        # simplified scenario reference: incoming grain is throughput-limited and weather-noisy.
        weather_bps=9000+rng.randrange(0,2001)
        incoming=100_000_000*s.route_capacity_bps//10000*weather_bps//10000
        consumption=92_000_000
        s.wd_grain=max(0,s.wd_grain+incoming-consumption)
        reserve=2_000_000_000
        scarcity=max(0,reserve-s.wd_grain)
        s.price_mcp=2000 + scarcity*3000//reserve + s.route_risk_bps//4
        s.tick += 288
    return s


def monte_carlo(snapshot,event,world_seed,run_count=100):
    prices=[]
    for i in range(run_count):
        r=run_from_snapshot(snapshot,event,30,derive_seed(world_seed,'blockade',i))
        prices.append(r.price_mcp)
    prices.sort()
    return {'runs':run_count,'min':prices[0],'p50':prices[len(prices)//2],'max':prices[-1],'mean':statistics.mean(prices)}


def main():
    main_state=State(550000,1_800_000_000,10000,1200,2300)
    main_before=state_hash(main_state)
    blockade={'start_day':0,'end_day':20,'route_capacity_bps':3500,'route_risk_bps':4200}
    child1=run_from_snapshot(main_state,blockade,30,derive_seed(12345,'scenario:blockade'))
    child2=run_from_snapshot(main_state,blockade,30,derive_seed(12345,'scenario:blockade'))
    no_blockade=run_from_snapshot(main_state,None,30,derive_seed(12345,'scenario:baseline'))
    assert state_hash(main_state)==main_before, 'canonical state mutated'
    assert state_hash(child1)==state_hash(child2), 'scenario replay not deterministic'
    assert state_hash(child1)!=state_hash(no_blockade), 'counterfactual did not diverge'
    assert child1.price_mcp>no_blockade.price_mcp
    mc=monte_carlo(main_state,blockade,12345,100)
    assert mc['min']<=mc['p50']<=mc['max']
    assert state_hash(main_state)==main_before
    print('SPRINT19_SCENARIOS: PASS')
    print('main_hash',main_before)
    print('blockade_final_price_mcp',child1.price_mcp)
    print('baseline_final_price_mcp',no_blockade.price_mcp)
    print('monte_carlo',mc)

if __name__=='__main__':main()
