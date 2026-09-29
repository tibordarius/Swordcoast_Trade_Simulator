import {pointAlongPolyline,polylineDistanceMeters} from './toril-gcs.mjs';

const BI=v=>typeof v==='bigint'?v:BigInt(v);

export function shipmentProgress(shipment,currentTick){
  const start=BI(shipment.departure_tick), end=BI(shipment.eta_tick), now=BI(currentTick);
  if(end<=start) return now>=end?1:0;
  if(now<=start) return 0;
  if(now>=end) return 1;
  const num=Number(now-start), den=Number(end-start);
  return num/den;
}

export function routeObject(route){
  return {
    id:route.id,
    coords:route.coordinates_fria.map(([lon_fria,lat])=>({lon_fria,lat})),
    distance_m:route.distance_m ?? Math.round(polylineDistanceMeters(route.coordinates_fria.map(([lon_fria,lat])=>({lon_fria,lat}))))
  };
}

export function shipmentPosition(shipment,route,currentTick){
  const r=routeObject(route);
  const progress=shipmentProgress(shipment,currentTick);
  return {...pointAlongPolyline(r.coords,progress),progress};
}

export function routeFlowWidth(volumeMilli,{min=1,max=14,referenceMilli=500_000_000n}={}){
  const v=Number(BI(volumeMilli));
  const ref=Number(referenceMilli);
  if(v<=0) return 0;
  const scaled=min+(max-min)*Math.min(1,Math.sqrt(v/ref));
  return Number(scaled.toFixed(3));
}

export function makeMarketsGeoJSON(exchanges){
  return {type:'FeatureCollection',features:exchanges.map(m=>({
    type:'Feature',
    id:m.market_id,
    geometry:{type:'Point',coordinates:[m.lon_fria,m.lat]},
    properties:{market_id:m.market_id,code:m.code,name:m.name,status:m.status,uncertainty_km:m.uncertainty_km,lon_md:m.lon_md}
  }))};
}

export function makeRoutesGeoJSON(routes,flowByRoute={}){
  return {type:'FeatureCollection',features:routes.map(r=>({
    type:'Feature',id:r.id,
    geometry:{type:'LineString',coordinates:r.coordinates_fria},
    properties:{id:r.id,from:r.from,to:r.to,distance_m:r.distance_m,geometry_status:r.geometry_status,flow_milli:String(flowByRoute[r.id]??0),flow_width:routeFlowWidth(flowByRoute[r.id]??0)}
  }))};
}

export function makeShipmentsGeoJSON(shipments,routesById,currentTick){
  return {type:'FeatureCollection',features:shipments.map(s=>{
    const r=routesById.get(s.route_id);
    if(!r) throw new Error(`route_not_found:${s.route_id}`);
    const p=shipmentPosition(s,r,currentTick);
    return {type:'Feature',id:s.id,geometry:{type:'Point',coordinates:[p.lon_fria,p.lat]},properties:{...s,progress:p.progress}};
  })};
}
