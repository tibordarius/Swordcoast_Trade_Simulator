const ALLOWED_EVENT_TYPES=new Set(['route_disruption','production_shock','demand_shock','tax_change','weather','military_requisition','port_disruption','custom_modifier']);
const ALLOWED_MODIFIERS=new Set([
  'route_capacity_bps','route_risk_bps','route_cost_bps','production_rate_bps',
  'consumption_rate_bps','tax_bps','port_capacity_bps','information_delay_ticks'
]);

export function validateWorldEvent(e){
  if(!e||typeof e!=='object') throw new Error('event_required');
  if(!ALLOWED_EVENT_TYPES.has(e.event_type)) throw new Error('unsupported_event_type');
  const start=BigInt(e.start_tick);
  const duration=e.duration_ticks==null?null:BigInt(e.duration_ticks);
  if(start<0n||(duration!==null&&duration<=0n)) throw new Error('bad_event_time');
  if(!Array.isArray(e.modifiers)||!e.modifiers.length) throw new Error('modifiers_required');
  for(const m of e.modifiers){
    if(!ALLOWED_MODIFIERS.has(m.type)) throw new Error(`forbidden_modifier:${m.type}`);
    if(typeof m.value_bps!=='number' && typeof m.value_ticks!=='string') throw new Error('modifier_value_required');
  }
  // Guardrail: world events may change causes of price, never price itself.
  const encoded=JSON.stringify(e).toLowerCase();
  if(encoded.includes('set_price')||encoded.includes('price_mcp')||encoded.includes('price_override')) throw new Error('direct_price_override_forbidden');
  return {...e,start_tick:start.toString(),duration_ticks:duration?.toString()??null};
}

export function previewEvent(event, world){
  const e=validateWorldEvent(event); const impacted=[];
  const routeIds=new Set(e.scope?.route_ids??[]), marketIds=new Set(e.scope?.market_ids??[]), siteIds=new Set(e.scope?.site_ids??[]);
  for(const r of world.routes??[]) if(routeIds.has(r.id)) impacted.push({kind:'route',id:r.id});
  for(const m of world.markets??[]) if(marketIds.has(m.id)) impacted.push({kind:'market',id:m.id});
  for(const s of world.production_sites??[]) if(siteIds.has(s.id)) impacted.push({kind:'production_site',id:s.id});
  return {event:e,impacted};
}

export function validateControlCommand(c,currentTick){
  if(!c||typeof c!=='object') throw new Error('command_required');
  const now=BigInt(currentTick);
  if(c.type==='pause'||c.type==='resume') return c;
  if(c.type==='set_speed'){
    const allowed=new Set([1,5,20,100,500]); if(!allowed.has(c.multiplier)) throw new Error('unsupported_speed'); return c;
  }
  if(c.type==='advance_to_tick'){
    const target=BigInt(c.tick); if(target<now) throw new Error('cannot_advance_backwards'); return {...c,tick:target.toString()};
  }
  throw new Error('unsupported_control_command');
}

/** Produce a trace from already-computed deterministic price components.
 * The function does not invent causes. Callers supply concrete evidence links.
 */
export function explainPriceMove({market_id,commodity_id,price_mcp,components,evidence=[]}){
  const ranked=Object.entries(components)
    .filter(([,v])=>Number.isInteger(v)&&v!==0)
    .map(([key,contribution_bps])=>({key,contribution_bps,abs:Math.abs(contribution_bps)}))
    .sort((a,b)=>b.abs-a.abs)
    .map(({abs,...x})=>x);
  return {
    market_id,commodity_id,price_mcp:String(price_mcp),
    factors:ranked.map(f=>({...f,evidence:evidence.filter(e=>e.factor===f.key)}))
  };
}
