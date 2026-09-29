import assert from 'node:assert/strict';

const U64_KEYS = new Set([
  'sequence','tick','world_id','branch_id','market_id','commodity_id',
  'bid_mcp','ask_mcp','on_hand_milli','reserved_milli','quantity_milli',
  'departure_tick','eta_tick'
]);

function assertDecimalStrings(value, path = '$') {
  if (Array.isArray(value)) {
    value.forEach((v,i) => assertDecimalStrings(v, `${path}[${i}]`));
    return;
  }
  if (!value || typeof value !== 'object') return;
  for (const [k,v] of Object.entries(value)) {
    if (U64_KEYS.has(k)) {
      assert.equal(typeof v, 'string', `${path}.${k} must be string`);
      assert.match(v, /^-?\d+$/, `${path}.${k} must be decimal`);
    }
    assertDecimalStrings(v, `${path}.${k}`);
  }
}

function applyBatch(client, batch) {
  const from = BigInt(batch.from_sequence);
  const to = BigInt(batch.to_sequence);
  const expected = client.sequence + 1n;

  if (to <= client.sequence) return 'duplicate';
  if (from !== expected) return 'gap';

  for (const d of batch.deltas) {
    const seq = BigInt(d.sequence);
    assert.equal(seq, client.sequence + 1n);
    if (d.type === 'market_state_changed') {
      const key = `${d.key.market_id}:${d.key.commodity_id}`;
      client.marketState.set(key, {
        bid_mcp: BigInt(d.data.bid_mcp),
        ask_mcp: BigInt(d.data.ask_mcp),
      });
    }
    client.sequence = seq;
  }
  client.tick = BigInt(batch.tick);
  assert.equal(client.sequence, to);
  return 'applied';
}

const snapshot = {
  protocol_version: 1,
  world_id: '1', branch_id: '1', simulation_version: '0.1.0',
  sequence: '8422', tick: '550230',
  markets: [{market_id:'4', commodity_id:'1', bid_mcp:'2000', ask_mcp:'2020', on_hand_milli:'9007199254740993000'}],
  shipments: []
};
assertDecimalStrings(snapshot);

const client = {
  sequence: BigInt(snapshot.sequence),
  tick: BigInt(snapshot.tick),
  marketState: new Map([['4:1',{bid_mcp:2000n,ask_mcp:2020n}]])
};

const batch1 = {
  protocol_version: 1, world_id:'1', branch_id:'1',
  from_sequence:'8423', to_sequence:'8424', tick:'550235',
  deltas:[
    {sequence:'8423',type:'market_state_changed',key:{market_id:'4',commodity_id:'1'},data:{bid_mcp:'2110',ask_mcp:'2136'}},
    {sequence:'8424',type:'market_state_changed',key:{market_id:'4',commodity_id:'1'},data:{bid_mcp:'2120',ask_mcp:'2147'}}
  ]
};
assertDecimalStrings(batch1);
assert.equal(applyBatch(client,batch1),'applied');
assert.equal(client.sequence,8424n);
assert.equal(client.marketState.get('4:1').ask_mcp,2147n);

// Duplicate frames are harmless.
assert.equal(applyBatch(client,batch1),'duplicate');
assert.equal(client.sequence,8424n);

// Missing 8425 must be detected rather than silently applying 8426.
const gap = {
  protocol_version:1,world_id:'1',branch_id:'1',
  from_sequence:'8426',to_sequence:'8426',tick:'550240',
  deltas:[{sequence:'8426',type:'market_state_changed',key:{market_id:'4',commodity_id:'1'},data:{bid_mcp:'2200',ask_mcp:'2230'}}]
};
assert.equal(applyBatch(client,gap),'gap');
assert.equal(client.sequence,8424n);

console.log('SPRINT8_LIVE_PROTOCOL: PASS');
console.log('last_sequence', client.sequence.toString());
console.log('tick', client.tick.toString());
console.log('ask_mcp', client.marketState.get('4:1').ask_mcp.toString());
