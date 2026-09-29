import test from 'node:test';
import assert from 'node:assert/strict';
import {createAccount,createWarehouse,createVessel,position,availableCash,executeMarketBuy,executeMarketSell,placeLimitOrder,tryFillLimitOrder,transferToVessel} from './trading.mjs';

const grain={id:'1',mass_mg_per_milli_unit:'1000',volume_mm3_per_milli_unit:'1333'};
const catalog=new Map([['1',grain]]);
const baseRow=()=>({market_id:'4',commodity_id:'1',ask_mcp:'2100',bid_mcp:'2080',on_hand_milli:'1000000000',depth_milli:'1000000000'});

test('market buy moves cash, market inventory and warehouse title atomically',()=>{
  const account=createAccount('1000000000');
  const warehouse=createWarehouse('1000000000000','2000000000000');
  const row=baseRow();
  const x=executeMarketBuy({account,warehouse,row,commodity:grain,commodityCatalog:catalog,quantityMilli:'100000000',expectedSequence:'42',currentSequence:'42'});
  assert.equal(position(warehouse,'1').quantity_milli,100000000n);
  assert.equal(row.on_hand_milli,'900000000');
  assert.equal(account.cash_mcp,1000000000n-BigInt(x.notional_mcp));
});

test('stale order preview cannot mutate state',()=>{
  const account=createAccount('1000000000');
  const warehouse=createWarehouse('1000000000000','2000000000000');
  const row=baseRow();
  assert.throws(()=>executeMarketBuy({account,warehouse,row,commodity:grain,commodityCatalog:catalog,quantityMilli:'1',expectedSequence:'41',currentSequence:'42'}),/stale_market_state/);
  assert.equal(account.cash_mcp,1000000000n);assert.equal(row.on_hand_milli,'1000000000');
});

test('limit buy reserves funds and fills only when executable average clears limit',()=>{
  const account=createAccount('1000000000');
  const warehouse=createWarehouse('1000000000000','2000000000000');
  const row=baseRow();
  const order=placeLimitOrder({id:'L1',side:'buy',account,warehouse,commodity:grain,quantityMilli:'100000000',limitPriceMcp:'2050',expectedSequence:'1',currentSequence:'1'});
  assert.equal(order.status,'open');assert(availableCash(account)<account.cash_mcp);
  assert.equal(tryFillLimitOrder({order,account,warehouse,row,commodity:grain,commodityCatalog:catalog}),null);
  row.ask_mcp='2000';row.bid_mcp='1980';
  const fill=tryFillLimitOrder({order,account,warehouse,row,commodity:grain,commodityCatalog:catalog});
  assert(fill);assert.equal(order.status,'filled');assert.equal(account.reserved_cash_mcp,0n);
});

test('owned warehouse goods can become physical vessel cargo',()=>{
  const warehouse=createWarehouse('1000000000000','2000000000000');
  const vessel=createVessel('50000000000','80000000000');
  position(warehouse,'1').quantity_milli=10000000n;
  transferToVessel({warehouse,vessel,commodity:grain,commodityCatalog:catalog,quantityMilli:'5000000'});
  assert.equal(position(warehouse,'1').quantity_milli,5000000n);
  assert.equal(position(vessel,'1').quantity_milli,5000000n);
});

test('market sell returns warehouse stock to market and credits account',()=>{
  const account=createAccount('0');
  const warehouse=createWarehouse('1000000000000','2000000000000');
  const row=baseRow();
  position(warehouse,'1').quantity_milli=50000000n;
  const x=executeMarketSell({account,warehouse,row,commodity:grain,quantityMilli:'30000000',expectedSequence:'9',currentSequence:'9'});
  assert.equal(position(warehouse,'1').quantity_milli,20000000n);
  assert.equal(row.on_hand_milli,'1030000000');
  assert.equal(account.cash_mcp,BigInt(x.notional_mcp));
});
