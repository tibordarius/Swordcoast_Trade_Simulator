import test from 'node:test';
import assert from 'node:assert/strict';
import {
  mcpToCpString,
  pctChangeBps,
  daysCover,
  createClientState,
  applyDeltaBatch,
  previewMarketOrder,
} from './market.mjs';

test('formats exact milli-copper without float loss', () => {
  assert.equal(mcpToCpString('9007199254740993000'), '9,007,199,254,740,993.000');
});

test('price change is integer-based', () => {
  assert.equal(pctChangeBps('2136', '1982'), 776);
});

test('days cover uses fixed point quantities', () => {
  assert.equal(daysCover('2520000000', '420000000'), 6);
});

test('delta gap is rejected', () => {
  const state = createClientState({ sequence: '10', tick: '100', markets: [] });
  const result = applyDeltaBatch(state, {
    from_sequence: '12',
    to_sequence: '12',
    tick: '105',
    deltas: [],
  });
  assert.equal(result.status, 'gap');
  assert.equal(state.sequence, 10n);
});

test('market order preview respects slippage', () => {
  const row = { ask_mcp: '2000', bid_mcp: '1980', depth_milli: '1000000000' };
  const preview = previewMarketOrder(row, 'buy', '500000000');
  assert.equal(preview.terminal_impact_bps, 100);
  assert.equal(preview.average_price_mcp, '2010');
  assert.equal(preview.notional_mcp, '1005000000');
});
