import {previewMarketOrder} from './market.mjs';

const toBI = v => typeof v === 'bigint' ? v : BigInt(v);

export function createAccount(cashMcp) {
  return {cash_mcp:toBI(cashMcp), reserved_cash_mcp:0n};
}

export function availableCash(account) {
  return account.cash_mcp - account.reserved_cash_mcp;
}

export function createWarehouse(maxMassMg, maxVolumeMm3) {
  return {max_mass_mg:toBI(maxMassMg), max_volume_mm3:toBI(maxVolumeMm3), positions:new Map()};
}

export function createVessel(maxMassMg, maxVolumeMm3) {
  return {max_mass_mg:toBI(maxMassMg), max_volume_mm3:toBI(maxVolumeMm3), positions:new Map()};
}

function getPosition(container, commodityId) {
  const key=String(commodityId);
  if(!container.positions.has(key)) container.positions.set(key,{quantity_milli:0n,reserved_milli:0n});
  return container.positions.get(key);
}

export function position(container, commodityId) {
  return getPosition(container, commodityId);
}

export function availablePosition(container, commodityId) {
  const p=getPosition(container,commodityId);
  return p.quantity_milli-p.reserved_milli;
}

export function containerUsage(container, commodityCatalog) {
  let mass=0n, volume=0n;
  for(const [commodityId,p] of container.positions){
    const c=commodityCatalog.get(String(commodityId));
    if(!c) throw new Error(`missing commodity ${commodityId}`);
    mass += p.quantity_milli * toBI(c.mass_mg_per_milli_unit);
    volume += p.quantity_milli * toBI(c.volume_mm3_per_milli_unit);
  }
  return {mass_mg:mass,volume_mm3:volume};
}

export function canAdd(container, commodity, quantityMilli, commodityCatalog) {
  const qty=toBI(quantityMilli);
  const usage=containerUsage(container,commodityCatalog);
  return usage.mass_mg + qty*toBI(commodity.mass_mg_per_milli_unit) <= container.max_mass_mg &&
         usage.volume_mm3 + qty*toBI(commodity.volume_mm3_per_milli_unit) <= container.max_volume_mm3;
}

function assertSequence(expectedSequence,currentSequence){
  if(toBI(expectedSequence)!==toBI(currentSequence)){
    const e=new Error('stale_market_state');e.code='stale_market_state';throw e;
  }
}

export function executeMarketBuy({account,warehouse,row,commodity,commodityCatalog,quantityMilli,expectedSequence,currentSequence}){
  assertSequence(expectedSequence,currentSequence);
  const qty=toBI(quantityMilli);
  if(qty<=0n) throw new Error('invalid_quantity');
  if(toBI(row.on_hand_milli)<qty) throw new Error('insufficient_market_inventory');
  if(!canAdd(warehouse,commodity,qty,commodityCatalog)) throw new Error('warehouse_capacity');
  const preview=previewMarketOrder(row,'buy',qty);
  const notional=toBI(preview.notional_mcp);
  if(availableCash(account)<notional) throw new Error('insufficient_cash');
  account.cash_mcp-=notional;
  getPosition(warehouse,commodity.id).quantity_milli+=qty;
  row.on_hand_milli=(toBI(row.on_hand_milli)-qty).toString();
  return {side:'buy',quantity_milli:qty.toString(),price_mcp:preview.average_price_mcp,notional_mcp:notional.toString()};
}

export function executeMarketSell({account,warehouse,row,commodity,quantityMilli,expectedSequence,currentSequence}){
  assertSequence(expectedSequence,currentSequence);
  const qty=toBI(quantityMilli);
  if(qty<=0n) throw new Error('invalid_quantity');
  if(availablePosition(warehouse,commodity.id)<qty) throw new Error('insufficient_owned_inventory');
  const preview=previewMarketOrder(row,'sell',qty);
  const notional=toBI(preview.notional_mcp);
  getPosition(warehouse,commodity.id).quantity_milli-=qty;
  account.cash_mcp+=notional;
  row.on_hand_milli=(toBI(row.on_hand_milli)+qty).toString();
  return {side:'sell',quantity_milli:qty.toString(),price_mcp:preview.average_price_mcp,notional_mcp:notional.toString()};
}

export function placeLimitOrder({id,side,account,warehouse,commodity,quantityMilli,limitPriceMcp,expectedSequence,currentSequence}){
  assertSequence(expectedSequence,currentSequence);
  const qty=toBI(quantityMilli), limit=toBI(limitPriceMcp);
  if(qty<=0n||limit<=0n) throw new Error('invalid_order');
  const order={id:String(id),side,status:'open',commodity_id:String(commodity.id),quantity_milli:qty,limit_price_mcp:limit,reserved_cash_mcp:0n};
  if(side==='buy'){
    const reserve=(limit*qty+999n)/1000n;
    if(availableCash(account)<reserve) throw new Error('insufficient_cash');
    account.reserved_cash_mcp+=reserve;order.reserved_cash_mcp=reserve;
  }else if(side==='sell'){
    if(availablePosition(warehouse,commodity.id)<qty) throw new Error('insufficient_owned_inventory');
    getPosition(warehouse,commodity.id).reserved_milli+=qty;
  }else throw new Error('invalid_side');
  return order;
}

export function cancelLimitOrder({order,account,warehouse}){
  if(order.status!=='open') return false;
  if(order.side==='buy') account.reserved_cash_mcp-=order.reserved_cash_mcp;
  else getPosition(warehouse,order.commodity_id).reserved_milli-=order.quantity_milli;
  order.status='cancelled';return true;
}

export function tryFillLimitOrder({order,account,warehouse,row,commodity,commodityCatalog}){
  if(order.status!=='open') return null;
  const qty=order.quantity_milli;
  if(order.side==='buy'){
    if(toBI(row.ask_mcp)>order.limit_price_mcp) return null;
    const preview=previewMarketOrder(row,'buy',qty);
    const notional=toBI(preview.notional_mcp);
    if(toBI(preview.average_price_mcp)>order.limit_price_mcp) return null;
    if(toBI(row.on_hand_milli)<qty) return null;
    if(!canAdd(warehouse,commodity,qty,commodityCatalog)) return null;
    account.reserved_cash_mcp-=order.reserved_cash_mcp;
    account.cash_mcp-=notional;
    getPosition(warehouse,commodity.id).quantity_milli+=qty;
    row.on_hand_milli=(toBI(row.on_hand_milli)-qty).toString();
    order.status='filled';
    return {side:'buy',quantity_milli:qty.toString(),price_mcp:preview.average_price_mcp,notional_mcp:notional.toString()};
  }
  if(toBI(row.bid_mcp)<order.limit_price_mcp) return null;
  const preview=previewMarketOrder(row,'sell',qty);
  if(toBI(preview.average_price_mcp)<order.limit_price_mcp) return null;
  const notional=toBI(preview.notional_mcp);
  const p=getPosition(warehouse,commodity.id);
  p.reserved_milli-=qty;p.quantity_milli-=qty;
  account.cash_mcp+=notional;
  row.on_hand_milli=(toBI(row.on_hand_milli)+qty).toString();
  order.status='filled';
  return {side:'sell',quantity_milli:qty.toString(),price_mcp:preview.average_price_mcp,notional_mcp:notional.toString()};
}

export function transferToVessel({warehouse,vessel,commodity,commodityCatalog,quantityMilli}){
  const qty=toBI(quantityMilli);
  if(availablePosition(warehouse,commodity.id)<qty) throw new Error('insufficient_owned_inventory');
  if(!canAdd(vessel,commodity,qty,commodityCatalog)) throw new Error('vessel_capacity');
  getPosition(warehouse,commodity.id).quantity_milli-=qty;
  getPosition(vessel,commodity.id).quantity_milli+=qty;
  return qty;
}
