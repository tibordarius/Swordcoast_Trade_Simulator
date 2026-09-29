import {
  mcpToCpString,
  milliToUnitString,
  pctChangeBps,
  formatBpsPct,
  daysCover,
  createClientState,
  previewMarketOrder,
} from './market.mjs';

const params = new URLSearchParams(location.search);
const apiBase = (params.get('api') ?? 'http://127.0.0.1:3000').replace(/\/$/, '');
const snapshot = await fetch('./mock-snapshot.json').then((response) => response.json());
const worldId = snapshot.world_id;
const state = createClientState(snapshot);
let selected = snapshot.markets[0];
let side = 'buy';

const rowsEl = document.querySelector('#marketRows');
const searchEl = document.querySelector('#search');
const connectionEl = document.querySelector('#connection');
const tickEl = document.querySelector('#tick');
const hashEl = document.querySelector('#stateHash');
const liveDot = document.querySelector('#liveDot');

document.querySelector('#sequence').textContent = state.sequence.toString();
tickEl.textContent = state.tick.toString();

function currentRows() {
  return [...state.markets.values()];
}

function renderRows(filter = '') {
  const query = filter.trim().toLowerCase();
  rowsEl.innerHTML = '';
  for (const row of currentRows().filter(
    (candidate) =>
      !query ||
      candidate.symbol.toLowerCase().includes(query) ||
      candidate.name.toLowerCase().includes(query),
  )) {
    const tr = document.createElement('tr');
    if (row.commodity_id === selected.commodity_id) tr.classList.add('selected');
    const change = pctChangeBps(row.ask_mcp, row.previous_close_mcp);
    const cover = daysCover(row.on_hand_milli, row.daily_demand_milli);
    tr.innerHTML = `<td><span class="symbol">${row.symbol}</span><span class="commodity">${row.name}</span></td>
      <td>${mcpToCpString(row.bid_mcp)}</td><td>${mcpToCpString(row.ask_mcp)}</td>
      <td class="${change > 0 ? 'up' : change < 0 ? 'down' : 'neutral'}">${formatBpsPct(change)}</td>
      <td>${milliToUnitString(row.on_hand_milli, 0)}</td><td>${cover?.toFixed(1) ?? '—'}</td>`;
    tr.onclick = () => {
      selected = row;
      renderRows(searchEl.value);
      renderDetail();
    };
    rowsEl.appendChild(tr);
  }
}

function renderDetail() {
  document.querySelector('#detailSymbol').textContent = selected.symbol;
  document.querySelector('#detailName').textContent = selected.name;
  document.querySelector('#detailBid').textContent = `${mcpToCpString(selected.bid_mcp)} cp`;
  document.querySelector('#detailAsk').textContent = `${mcpToCpString(selected.ask_mcp)} cp`;
  document.querySelector('#detailStock').textContent = milliToUnitString(selected.on_hand_milli, 0);
  document.querySelector('#detailReserve').textContent = milliToUnitString(selected.target_reserve_milli, 0);
  document.querySelector('#detailCover').textContent =
    `${daysCover(selected.on_hand_milli, selected.daily_demand_milli).toFixed(1)} days`;
  document.querySelector('#detailVolume').textContent = milliToUnitString(selected.volume_milli, 0);
  renderPreview();
}

function renderPreview() {
  const units = document.querySelector('#quantity').value || '0';
  const milli = BigInt(Math.max(0, Math.round(Number(units) * 1000)));
  if (milli === 0n) {
    document.querySelector('#previewPrice').textContent = '—';
    return;
  }
  const preview = previewMarketOrder(selected, side, milli.toString());
  document.querySelector('#previewPrice').textContent =
    `${mcpToCpString(preview.average_price_mcp)} cp`;
  document.querySelector('#previewImpact').textContent =
    `${(preview.terminal_impact_bps / 100).toFixed(2)}%`;
  document.querySelector('#previewNotional').textContent =
    `${mcpToCpString(preview.notional_mcp)} cp`;
}

function applyEngineStatus(status) {
  tickEl.textContent = status.tick;
  hashEl.textContent = status.state_hash;
  connectionEl.textContent = 'ENGINE LIVE';
  liveDot.classList.remove('offline');
}

function setOffline() {
  connectionEl.textContent = 'REFERENCE OFFLINE';
  liveDot.classList.add('offline');
}

async function fetchStatus() {
  const response = await fetch(`${apiBase}/v1/worlds/${encodeURIComponent(worldId)}/status`);
  if (!response.ok) throw new Error(`status HTTP ${response.status}`);
  applyEngineStatus(await response.json());
}

async function advance(ticks) {
  const response = await fetch(
    `${apiBase}/v1/worlds/${encodeURIComponent(worldId)}/advance`,
    {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ ticks: String(ticks) }),
    },
  );
  if (!response.ok) throw new Error(`advance HTTP ${response.status}`);
  applyEngineStatus(await response.json());
}

function connectLive() {
  const wsBase = apiBase.replace(/^http:/, 'ws:').replace(/^https:/, 'wss:');
  const socket = new WebSocket(`${wsBase}/v1/live/${encodeURIComponent(worldId)}`);
  socket.onmessage = (event) => applyEngineStatus(JSON.parse(event.data));
  socket.onopen = () => {
    connectionEl.textContent = 'ENGINE LIVE';
    liveDot.classList.remove('offline');
  };
  socket.onerror = setOffline;
  socket.onclose = () => {
    setOffline();
    setTimeout(connectLive, 2000);
  };
}

searchEl.oninput = (event) => renderRows(event.target.value);
document.querySelector('#quantity').oninput = renderPreview;
document.querySelectorAll('[data-side]').forEach((button) => {
  button.onclick = () => {
    side = button.dataset.side;
    document.querySelectorAll('[data-side]').forEach(
      (candidate) => candidate.classList.toggle('active', candidate === button),
    );
    renderPreview();
  };
});
document.querySelectorAll('[data-advance]').forEach((button) => {
  button.onclick = async () => {
    button.disabled = true;
    try {
      await advance(button.dataset.advance);
    } catch (error) {
      console.error(error);
      setOffline();
    } finally {
      button.disabled = false;
    }
  };
});

renderRows();
renderDetail();

try {
  await fetchStatus();
  connectLive();
} catch (error) {
  console.warn('WDEX engine unavailable, using reference snapshot only', error);
  setOffline();
}
