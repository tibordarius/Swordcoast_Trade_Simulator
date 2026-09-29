import {mcpToCpString,milliToUnitString,pctChangeBps,formatBpsPct,daysCover,createClientState,previewMarketOrder} from './market.mjs';

let snapshot = await fetch('./mock-snapshot.json').then(r=>r.json());
let state = createClientState(snapshot);
let selected = snapshot.markets[0];
let side = 'buy';

const rowsEl=document.querySelector('#marketRows');
const searchEl=document.querySelector('#search');
document.querySelector('#sequence').textContent=state.sequence.toString();
document.querySelector('#tick').textContent=state.tick.toString();

function currentRows(){ return [...state.markets.values()]; }
function renderRows(filter=''){
  const q=filter.trim().toLowerCase();
  rowsEl.innerHTML='';
  for(const row of currentRows().filter(r=>!q||r.symbol.toLowerCase().includes(q)||r.name.toLowerCase().includes(q))){
    const tr=document.createElement('tr');
    if(row.commodity_id===selected.commodity_id) tr.classList.add('selected');
    const change=pctChangeBps(row.ask_mcp,row.previous_close_mcp);
    const cover=daysCover(row.on_hand_milli,row.daily_demand_milli);
    tr.innerHTML=`<td><span class="symbol">${row.symbol}</span><span class="commodity">${row.name}</span></td>
      <td>${mcpToCpString(row.bid_mcp)}</td><td>${mcpToCpString(row.ask_mcp)}</td>
      <td class="${change>0?'up':change<0?'down':'neutral'}">${formatBpsPct(change)}</td>
      <td>${milliToUnitString(row.on_hand_milli,0)}</td><td>${cover?.toFixed(1)??'—'}</td>`;
    tr.onclick=()=>{selected=row;renderRows(searchEl.value);renderDetail();};
    rowsEl.appendChild(tr);
  }
}
function renderDetail(){
  document.querySelector('#detailSymbol').textContent=selected.symbol;
  document.querySelector('#detailName').textContent=selected.name;
  document.querySelector('#detailBid').textContent=mcpToCpString(selected.bid_mcp)+' cp';
  document.querySelector('#detailAsk').textContent=mcpToCpString(selected.ask_mcp)+' cp';
  document.querySelector('#detailStock').textContent=milliToUnitString(selected.on_hand_milli,0);
  document.querySelector('#detailReserve').textContent=milliToUnitString(selected.target_reserve_milli,0);
  document.querySelector('#detailCover').textContent=daysCover(selected.on_hand_milli,selected.daily_demand_milli).toFixed(1)+' days';
  document.querySelector('#detailVolume').textContent=milliToUnitString(selected.volume_milli,0);
  renderPreview();
}
function renderPreview(){
  const units=document.querySelector('#quantity').value||'0';
  const milli=BigInt(Math.max(0,Math.round(Number(units)*1000)));
  if(milli===0n){document.querySelector('#previewPrice').textContent='—';return;}
  const p=previewMarketOrder(selected,side,milli.toString());
  document.querySelector('#previewPrice').textContent=mcpToCpString(p.average_price_mcp)+' cp';
  document.querySelector('#previewImpact').textContent=(p.terminal_impact_bps/100).toFixed(2)+'%';
  document.querySelector('#previewNotional').textContent=mcpToCpString(p.notional_mcp)+' cp';
}
searchEl.oninput=e=>renderRows(e.target.value);
document.querySelector('#quantity').oninput=renderPreview;
document.querySelectorAll('[data-side]').forEach(btn=>btn.onclick=()=>{
  side=btn.dataset.side;
  document.querySelectorAll('[data-side]').forEach(b=>b.classList.toggle('active',b===btn));
  renderPreview();
});
renderRows();renderDetail();
