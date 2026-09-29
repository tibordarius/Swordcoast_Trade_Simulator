import test from 'node:test';import assert from 'node:assert/strict';
import {validateWorldEvent,previewEvent,validateControlCommand,explainPriceMove} from './control.mjs';
const blockade={event_id:'EV-1',event_type:'route_disruption',start_tick:'1000',duration_ticks:'2016',scope:{route_ids:['SEA-ATH-BG']},modifiers:[{type:'route_capacity_bps',value_bps:-6000},{type:'route_risk_bps',value_bps:2200}],visibility:'rumor'};
test('blockade previews concrete route without price mutation',()=>{
 const p=previewEvent(blockade,{routes:[{id:'SEA-ATH-BG'},{id:'SEA-BG-WD'}]});
 assert.deepEqual(p.impacted,[{kind:'route',id:'SEA-ATH-BG'}]);
});
test('direct price overrides are rejected',()=>assert.throws(()=>validateWorldEvent({...blockade,modifiers:[{type:'price_mcp',value_bps:100}]}),/forbidden_modifier|price/));
test('advance cannot move campaign time backwards',()=>assert.throws(()=>validateControlCommand({type:'advance_to_tick',tick:'9'},'10'),/backwards/));
test('causal explanation ranks deterministic contributions and preserves evidence',()=>{
 const x=explainPriceMove({market_id:'WDEX',commodity_id:'GRAIN',price_mcp:'5032',components:{reserve_bps:8000,incoming_bps:3500,risk_bps:1200,speculation_bps:500},evidence:[{factor:'incoming_bps',kind:'shipment',id:'SHIP-77'},{factor:'risk_bps',kind:'event',id:'EV-1',visibility:'rumor'}]});
 assert.equal(x.factors[0].key,'reserve_bps');assert.equal(x.factors[1].evidence[0].id,'SHIP-77');
});
