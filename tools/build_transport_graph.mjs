#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {vincentyDistanceMeters} from '../code/web/map/toril-gcs.mjs';

const __dirname=path.dirname(fileURLToPath(import.meta.url));
const ROOT=path.resolve(__dirname,'..');

export function nodeKey([lon,lat],precision=6){return `${Number(lon).toFixed(precision)},${Number(lat).toFixed(precision)}`;}

export function buildTransportGraph(pathways,{precision=6,includeClasses=['Primary Road']}={}){
  const nodes=new Map();
  const edges=[];
  const adjacency=new Map();
  const ensure=(coord)=>{
    const key=nodeKey(coord,precision);
    if(!nodes.has(key)) nodes.set(key,{id:key,lon_fria:+Number(coord[0]).toFixed(precision),lat:+Number(coord[1]).toFixed(precision)});
    if(!adjacency.has(key)) adjacency.set(key,[]);
    return key;
  };
  for(const f of pathways.features||[]){
    const cls=f.properties?.feature_class||'Unknown';
    if(includeClasses && !includeClasses.includes(cls)) continue;
    if(f.geometry?.type!=='LineString') continue;
    const coords=f.geometry.coordinates||[];
    for(let i=1;i<coords.length;i++){
      const a=ensure(coords[i-1]), b=ensure(coords[i]);
      if(a===b) continue;
      const distance_m=Math.round(vincentyDistanceMeters(
        {lon_fria:nodes.get(a).lon_fria,lat:nodes.get(a).lat},
        {lon_fria:nodes.get(b).lon_fria,lat:nodes.get(b).lat}
      ));
      const edge={id:`${f.properties?.uuid||'path'}:${i-1}`,a,b,distance_m,feature_uuid:f.properties?.uuid||null,feature_class:cls};
      edges.push(edge);
      adjacency.get(a).push({to:b,edge_id:edge.id,distance_m});
      adjacency.get(b).push({to:a,edge_id:edge.id,distance_m});
    }
  }
  return {schema_version:1,crs:'Toril GCS / FRIA',nodes:[...nodes.values()],edges,adjacency:Object.fromEntries(adjacency)};
}

export function nearestGraphNode(graph,point,maxDistanceM=30000){
  let best=null;
  for(const n of graph.nodes){
    const d=vincentyDistanceMeters(point,{lon_fria:n.lon_fria,lat:n.lat});
    if(!best||d<best.distance_m) best={node_id:n.id,distance_m:Math.round(d)};
  }
  return best&&best.distance_m<=maxDistanceM?best:null;
}

export function shortestPath(graph,startId,endId){
  const dist=new Map([[startId,0]]), prev=new Map(), q=new Set(graph.nodes.map(n=>n.id));
  while(q.size){
    let u=null,best=Infinity;
    for(const id of q){const d=dist.get(id)??Infinity;if(d<best){best=d;u=id;}}
    if(u===null||best===Infinity) break;
    q.delete(u); if(u===endId) break;
    for(const e of graph.adjacency[u]||[]){
      if(!q.has(e.to)) continue;
      const alt=best+e.distance_m;
      if(alt<(dist.get(e.to)??Infinity)){dist.set(e.to,alt);prev.set(e.to,{from:u,edge_id:e.edge_id});}
    }
  }
  if(!dist.has(endId)) return null;
  const nodes=[]; const edges=[]; let cur=endId;
  while(cur!==startId){nodes.push(cur);const p=prev.get(cur);if(!p)return null;edges.push(p.edge_id);cur=p.from;}
  nodes.push(startId);nodes.reverse();edges.reverse();
  return {distance_m:dist.get(endId),nodes,edges};
}

function main(){
  const input=process.argv[2]||path.join(ROOT,'code/data/toril/cache/srf_civ_pathways_ln.sword-coast.geojson');
  const output=process.argv[3]||path.join(ROOT,'seed/toril_pathway_graph_v1.json');
  if(!fs.existsSync(input)){
    console.error(`Missing ${input}. Run: python tools/import_toril_gis.py`);process.exit(2);
  }
  const fc=JSON.parse(fs.readFileSync(input,'utf8'));
  const graph=buildTransportGraph(fc);
  graph.source={repo:'geospatial-grimoire/toril-gis',commit:'8f9bffae356ec32ec6f76ce788abf700d6e56c3b',layer:'srf_civ_pathways_ln'};
  fs.writeFileSync(output,JSON.stringify(graph,null,2)+'\n');
  console.log(JSON.stringify({nodes:graph.nodes.length,edges:graph.edges.length,output},null,2));
}

if(import.meta.url===`file://${process.argv[1]}`) main();
