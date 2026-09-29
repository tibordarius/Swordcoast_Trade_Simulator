import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {polylineDistanceMeters} from '../code/web/map/toril-gcs.mjs';

const here=path.dirname(fileURLToPath(import.meta.url));
const root=path.resolve(here,'..');
const markets=JSON.parse(fs.readFileSync(path.join(root,'seed/toril_exchange_coordinates_v1.json'),'utf8'));
const routes=JSON.parse(fs.readFileSync(path.join(root,'seed/routes_v0.json'),'utf8'));
const byId=new Map(markets.exchanges.map(x=>[x.market_id,x]));
const out={schema_version:1,coordinate_system:'Toril GCS / FRIA',upstream:markets.upstream,routes:[]};
for(const r of routes.routes){
  const a=byId.get(r.from),b=byId.get(r.to);
  if(!a||!b) throw new Error(`missing coordinate for ${r.id}`);
  const coords=[{lon_fria:a.lon_fria,lat:a.lat},{lon_fria:b.lon_fria,lat:b.lat}];
  out.routes.push({
    id:r.id,from:r.from,to:r.to,mode:r.mode,bidirectional:r.bidirectional,
    geometry_status:(a.status==='toril_gis_exact'&&b.status==='toril_gis_exact')?'direct_geodesic_exact_endpoints':'direct_geodesic_with_provisional_endpoint',
    coordinate_uncertainty_km:Math.max(a.uncertainty_km||0,b.uncertainty_km||0),
    distance_m:Math.round(polylineDistanceMeters(coords)),
    coordinates_fria:coords.map(p=>[p.lon_fria,p.lat]),
    note:'Straight geodesic placeholder. Replace with navigable sea-lane polyline before route distance becomes authoritative for freight/travel.'
  });
}
fs.writeFileSync(path.join(root,'seed/toril_route_geometries_v1.json'),JSON.stringify(out,null,2)+'\n');
console.log(JSON.stringify({status:'PASS',routes:out.routes.map(r=>({id:r.id,distance_km:Number((r.distance_m/1000).toFixed(1)),status:r.geometry_status}))},null,2));
