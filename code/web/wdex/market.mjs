export function mcpToCpString(value, decimals = 3) {
  const v = typeof value === 'bigint' ? value : BigInt(value);
  const sign = v < 0n ? '-' : '';
  const a = v < 0n ? -v : v;
  const whole = a / 1000n;
  const frac = (a % 1000n).toString().padStart(3, '0').slice(0, decimals);
  return `${sign}${whole.toLocaleString('en-US')}${decimals ? '.' + frac : ''}`;
}

export function milliToUnitString(value, decimals = 1) {
  const v = typeof value === 'bigint' ? value : BigInt(value);
  const sign = v < 0n ? '-' : '';
  const a = v < 0n ? -v : v;
  const whole = a / 1000n;
  const frac = (a % 1000n).toString().padStart(3, '0').slice(0, decimals);
  return `${sign}${whole.toLocaleString('en-US')}${decimals ? '.' + frac : ''}`;
}

export function pctChangeBps(currentMcp, previousMcp) {
  const current = BigInt(currentMcp);
  const previous = BigInt(previousMcp);
  if (previous === 0n) return 0;
  return Number(((current - previous) * 10_000n) / previous);
}

export function formatBpsPct(bps) {
  const sign = bps > 0 ? '+' : '';
  return `${sign}${(bps / 100).toFixed(2)}%`;
}

export function daysCover(onHandMilli, dailyDemandMilli) {
  const stock = BigInt(onHandMilli);
  const demand = BigInt(dailyDemandMilli);
  if (demand <= 0n) return null;
  return Number((stock * 100n) / demand) / 100;
}

export function marketKey(marketId, commodityId) {
  return `${marketId}:${commodityId}`;
}

export function createClientState(snapshot) {
  const markets = new Map();
  for (const row of snapshot.markets) {
    markets.set(marketKey(row.market_id, row.commodity_id), { ...row });
  }
  return {
    sequence: BigInt(snapshot.sequence),
    tick: BigInt(snapshot.tick),
    markets,
  };
}

export function applyDeltaBatch(state, batch) {
  const from = BigInt(batch.from_sequence);
  const to = BigInt(batch.to_sequence);
  if (to <= state.sequence) return { status: 'duplicate' };
  if (from !== state.sequence + 1n) {
    return { status: 'gap', expected: (state.sequence + 1n).toString() };
  }

  for (const d of batch.deltas) {
    const seq = BigInt(d.sequence);
    if (seq !== state.sequence + 1n) {
      return { status: 'gap', expected: (state.sequence + 1n).toString() };
    }
    if (d.type === 'market_state_changed' || d.type === 'market_inventory_changed') {
      const key = marketKey(d.key.market_id, d.key.commodity_id);
      const existing = state.markets.get(key) ?? {
        market_id: d.key.market_id,
        commodity_id: d.key.commodity_id,
      };
      state.markets.set(key, { ...existing, ...d.data });
    }
    state.sequence = seq;
  }
  state.tick = BigInt(batch.tick);
  return { status: 'applied' };
}

export function previewMarketOrder(row, side, quantityMilli) {
  const qty = BigInt(quantityMilli);
  const depth = BigInt(row.depth_milli);
  if (qty <= 0n) throw new Error('quantity must be positive');
  const rawImpact = (qty * 200n) / depth;
  const terminalImpactBps = Number(rawImpact > 5000n ? 5000n : rawImpact);
  const averageImpactBps = BigInt(Math.trunc(terminalImpactBps / 2));
  const base = BigInt(side === 'buy' ? row.ask_mcp : row.bid_mcp);
  const avg =
    side === 'buy'
      ? (base * (10_000n + averageImpactBps) + 5_000n) / 10_000n
      : (base * (10_000n - averageImpactBps) + 5_000n) / 10_000n;
  const notionalMcp = (avg * qty + 500n) / 1000n;
  return {
    average_price_mcp: avg.toString(),
    terminal_impact_bps: terminalImpactBps,
    notional_mcp: notionalMcp.toString(),
  };
}
