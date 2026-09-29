import {previewMarketOrder} from './market.mjs';
const BI=v=>typeof v==='bigint'?v:BigInt(v);

function directedEdges(routes){
  const out=[];
  for(const r of routes){
    out.push({...r});
    if(r.bidirectional) out.push({...r,id:r.id+':REV',from:r.to,to:r.from});
  }
  return out;
}

export function combinedRiskBps(path){
  let survival=10_000n;
  for(const e of path) survival=(survival*BI(10_000-e.risk_bps)+5_000n)/10_000n;
  return Number(10_000n-survival);
}

export function routeTotals(path){
  return {
    days:path.reduce((a,e)=>a+e.days,0),
    freight_mcp_per_unit:path.reduce((a,e)=>a+BI(e.freight_mcp_per_unit),0n),
    risk_bps:combinedRiskBps(path),
    capacity_milli:path.reduce((a,e)=>a===null||BI(e.capacity_milli)<a?BI(e.capacity_milli):a,null)
  };
}

export function cheapestPath(routes,from,to,purchasePriceMcp,quantityMilli){
  const edges=directedEdges(routes);
  const nodes=new Set(edges.flatMap(e=>[e.from,e.to]));
  const dist=new Map([...nodes].map(n=>[n,null]));
  const paths=new Map();dist.set(String(from),0n);paths.set(String(from),[]);
  const visited=new Set();
  while(visited.size<nodes.size){
    let u=null,best=null;
    for(const [n,d] of dist){if(!visited.has(n)&&d!==null&&(best===null||d<best)){u=n;best=d;}}
    if(u===null) break;if(u===String(to)) break;visited.add(u);
    for(const e of edges.filter(x=>x.from===u)){
      if(BI(e.capacity_milli)<BI(quantityMilli)) continue;
      const riskCost=(BI(purchasePriceMcp)*BI(e.risk_bps)+5_000n)/10_000n;
      const edgeCost=BI(e.freight_mcp_per_unit)+riskCost;
      const nd=best+edgeCost;
      if(dist.get(e.to)===null||nd<dist.get(e.to)){dist.set(e.to,nd);paths.set(e.to,[...(paths.get(u)||[]),e]);}
    }
  }
  return paths.get(String(to))||null;
}

export function evaluateArbitrage(network,quantityMilli){
  const qty=BI(quantityMilli);const opportunities=[];
  for(const origin of network.markets){for(const destination of network.markets){
    if(origin.market_id===destination.market_id) continue;
    const path=cheapestPath(network.routes,origin.market_id,destination.market_id,origin.ask_mcp,qty);
    if(!path) continue;
    const rt=routeTotals(path);
    const buy=previewMarketOrder(origin,'buy',qty);
    const sell=previewMarketOrder(destination,'sell',qty);
    const purchase=BI(buy.notional_mcp), revenue=BI(sell.notional_mcp);
    const freight=(rt.freight_mcp_per_unit*qty+500n)/1000n;
    const risk=(purchase*BI(rt.risk_bps)+5_000n)/10_000n;
    const capital=purchase+freight+risk;
    const profit=revenue-capital;
    const roi_bps=capital>0n?Number((profit*10_000n)/capital):0;
    opportunities.push({
      origin:origin.code,destination:destination.code,
      origin_market_id:origin.market_id,destination_market_id:destination.market_id,
      path:path.map(e=>e.id.replace(':REV','')),
      days:rt.days,risk_bps:rt.risk_bps,
      capacity_milli:rt.capacity_milli.toString(),
      purchase_mcp:purchase.toString(),revenue_mcp:revenue.toString(),freight_mcp:freight.toString(),expected_risk_loss_mcp:risk.toString(),profit_mcp:profit.toString(),roi_bps
    });
  }}
  return opportunities.sort((a,b)=>b.roi_bps-a.roi_bps);
}
