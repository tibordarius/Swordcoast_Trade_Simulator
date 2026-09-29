#!/usr/bin/env python3
import hashlib,json,copy

def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':')).encode()
def h(x):return hashlib.sha256(canonical(x)).hexdigest()
def event_hash(prev,e):return hashlib.sha256((prev+h(e)).encode()).hexdigest()

def apply(s,e):
 s=copy.deepcopy(s)
 if e['type']=='inventory_delta':s['grain']+=e['delta']
 elif e['type']=='cash_delta':s['cash']+=e['delta']
 elif e['type']=='advance':s['tick']=e['tick']
 else:raise ValueError(e['type'])
 if s['grain']<0 or s['cash']<0:raise AssertionError('negative state')
 return s

def main():
 base={'tick':100,'grain':1_000_000,'cash':500_000}
 events=[{'seq':1,'type':'inventory_delta','delta':250_000},{'seq':2,'type':'cash_delta','delta':-100_000},{'seq':3,'type':'advance','tick':144},{'seq':4,'type':'inventory_delta','delta':-400_000}]
 state=copy.deepcopy(base);chain='GENESIS';logged=[]
 for e in events:
  chain=event_hash(chain,e);logged.append((copy.deepcopy(e),chain));state=apply(state,e)
 final_hash=h(state)
 # replay from same snapshot/event log
 replay=copy.deepcopy(base);prev='GENESIS'
 for e,expected in logged:
  actual=event_hash(prev,e);assert actual==expected;prev=actual;replay=apply(replay,e)
 assert h(replay)==final_hash
 # tamper detection
 bad=copy.deepcopy(logged);bad[1][0]['delta']=-100_001
 prev='GENESIS';detected=False
 for e,expected in bad:
  actual=event_hash(prev,e)
  if actual!=expected:detected=True;break
  prev=actual
 assert detected
 print('SPRINT20_RECOVERY: PASS')
 print('final_state_hash',final_hash)
 print('event_chain_head',chain)
 print('tamper_detected',detected)
if __name__=='__main__':main()
