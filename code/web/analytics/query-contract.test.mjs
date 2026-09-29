import test from 'node:test'; import assert from 'node:assert/strict';
import {validateAnalyticsRequest} from './query-contract.mjs';
test('large ticks remain decimal strings',()=>{
 const q=validateAnalyticsRequest({type:'price_history',branch_id:'MAIN',commodity_id:'GRAIN',min_tick:'9007199254740993',max_tick:'9007199254741993'});
 assert.equal(q.min_tick,'9007199254740993');
});
test('arbitrary query types are rejected',()=>assert.throws(()=>validateAnalyticsRequest({type:'sql',branch_id:'MAIN'}),/unsupported/));
