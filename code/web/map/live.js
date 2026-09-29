import {shipmentPosition,routeFlowWidth} from './live-logistics.mjs';

const [coordData,routeData,demo]=await Promise.all([
  fetch('../../../seed/toril_exchange_coordinates_v1.json').then(r=>r.json()),
  fetch('../../../seed/toril_route_geometries_v1.json').then(r=>r.json()),
  fetch('./live-demo.json').then(r=>r.json())
]);
const markets=coordData.exchanges;
const routes=routeData.routes;
const routesById=new Map(routes.map(x=>[x.id,x]));
const marketById=new Map(markets.map(x=>[x.market_id,x]));
const svg=document.querySelector('#map');
const tick=document.querySelector('#tick');
const tickValue=document.querySelector('#tick-value');
const list=document.querySelector('#shipments');
const NS='http://www.w3.org/2000/svg';
const bounds={west:-79,east:-66,south:21,north:53};
function project(lon,lat){return {x:(lon-bounds.west)/(bounds.east-bounds.west)*1000,y:(bounds.north-lat)/(bounds.north-bounds.south)*900};}
function el(name,attrs={}){const x=document.createElementNS(NS,name);for(const[k,v]of Object.entries(attrs))x.setAttribute(k,v);return x;}
function render(now){
  svg.replaceChildren();
  for(const r of routes){
    const a=marketById.get(r.from),b=marketById.get(r.to); if(!a||!b) continue;
    const pa=project(a.lon_fria,a.lat),pb=project(b.lon_fria,b.lat);
    const line=el('line',{x1:pa.x,y1:pa.y,x2:pb.x,y2:pb.y,class:`route ${r.geometry_status.includes('provisional')?'provisional':''}`,'stroke-width':routeFlowWidth(demo.flow_by_route[r.id]??0)});svg.append(line);
  }
  for(const m of markets){const p=project(m.lon_fria,m.lat);svg.append(el('circle',{cx:p.x,cy:p.y,r:7,class:'market'}));const t=el('text',{x:p.x+10,y:p.y-9,class:'label'});t.textContent=m.code;svg.append(t);}
  list.replaceChildren();
  for(const s of demo.shipments){const r=routesById.get(s.route_id);const p=shipmentPosition(s,r,String(now));const xy=project(p.lon_fria,p.lat);svg.append(el('circle',{cx:xy.x,cy:xy.y,r:5,class:'shipment'}));const div=document.createElement('div');div.className='ship';div.innerHTML=`<strong>${s.id} · ${s.commodity}</strong><small>${s.owner} · ${r.from} → ${r.to}</small><div class="bar"><i style="width:${(p.progress*100).toFixed(1)}%"></i></div>`;list.append(div);}
  tickValue.textContent=String(now);
}
tick.addEventListener('input',()=>render(Number(tick.value)));
render(Number(tick.value));
