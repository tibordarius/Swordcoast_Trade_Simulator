import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {evaluateArbitrage,cheapestPath,routeTotals} from './network.mjs';
const network=JSON.parse(fs.readFileSync(new URL('./mock-network.json',import.meta.url)));

test('all six exchanges participate in network model',()=>{
  assert.deepEqual(network.markets.map(x=>x.code),['CALEX','ATHEX','BGEX','WDEX','NWEX','LUSEX']);
});

test('route selection can choose multi-hop path',()=>{
  const p=cheapestPath(network.routes,'2','6','1660','100000000');
  assert(p);const t=routeTotals(p);assert(t.days>0);assert(p.length>=2);
});

test('executable arbitrage includes slippage freight and expected loss',()=>{
  const opp=evaluateArbitrage(network,'100000000');
  assert(opp.length>0);
  const best=opp[0];
  assert.equal(best.origin,'ATHEX');
  assert.equal(best.destination,'LUSEX');
  assert(best.roi_bps>0);
  assert(BigInt(best.freight_mcp)>0n);
  assert(BigInt(best.expected_risk_loss_mcp)>0n);
  assert(best.path.length>=2);
});

test('route capacity can make a theoretical spread untradeable',()=>{
  const huge=evaluateArbitrage(network,'700000000');
  assert.equal(huge.length,0);
});
