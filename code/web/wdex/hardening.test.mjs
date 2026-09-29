import test from 'node:test';import assert from 'node:assert/strict';
import {createAccount,createWarehouse,executeMarketBuy,executeMarketSell,position,availableCash} from './trading.mjs';

function rng(seed=0x12345678){let s=seed>>>0;return()=>{s^=s<<13;s^=s>>>17;s^=s<<5;return s>>>0}}
const commodity={id:'CMD-GRAIN',mass_mg_per_milli_unit:'1000',volume_mm3_per_milli_unit:'1000'};
const catalog=new Map([[commodity.id,commodity]]);

test('2000 deterministic random trades preserve non-negative ledgers',()=>{
 const r=rng();const account=createAccount('1000000000000');const wh=createWarehouse('999999999999999999','999999999999999999');
 const row={bid_mcp:'1992',ask_mcp:'2008',on_hand_milli:'5000000000',depth_milli:'1000000000',impact_bps_at_depth:'500'};
 for(let i=0;i<2000;i++){
   const owned=position(wh,commodity.id).quantity_milli;
   const market=BigInt(row.on_hand_milli);
   const buy=(r()&1)===0 || owned===0n;
   const raw=BigInt(1+(r()%50000))*1000n;
   if(buy){
     const qty=raw<market?raw:market;
     if(qty>0n){try{executeMarketBuy({account,warehouse:wh,row,commodity,commodityCatalog:catalog,quantityMilli:qty,expectedSequence:'1',currentSequence:'1'})}catch(e){if(!['insufficient_cash','insufficient_market_inventory'].includes(e.message))throw e;}}
   }else{
     const qty=raw<owned?raw:owned;
     if(qty>0n) executeMarketSell({account,warehouse:wh,row,commodity,quantityMilli:qty,expectedSequence:'1',currentSequence:'1'});
   }
   assert(BigInt(row.on_hand_milli)>=0n);assert(account.cash_mcp>=0n);assert(account.reserved_cash_mcp>=0n);assert(availableCash(account)>=0n);
   const p=position(wh,commodity.id);assert(p.quantity_milli>=0n);assert(p.reserved_milli>=0n);assert(p.reserved_milli<=p.quantity_milli);
 }
});
