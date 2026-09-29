import assert from 'node:assert/strict';
import {buildTransportGraph,nearestGraphNode,shortestPath} from '../tools/build_transport_graph.mjs';

// Exact Toril GIS pathway snippets for the north Sword Coast, pinned to commit 8f9bffae...
const fc={type:'FeatureCollection',features:[
 {type:'Feature',properties:{uuid:'8b17dc4b-3c1a-4dd2-b67b-e5b54419035a',feature_class:'Primary Road'},geometry:{type:'LineString',coordinates:[
 [-76.36662,51.34419],[-76.34529,51.16338],[-76.28778,50.85405],[-76.2543,50.62662],[-76.27032,50.48667],[-76.28175,50.3622],[-76.29651,50.2164],[-76.28976,50.09724],[-76.20156,49.96755],[-76.07835,49.81275],[-75.978,49.57281],[-75.91491,49.30596],[-75.74382,49.20768],[-75.5955,49.0455],[-75.4965,48.8475],[-75.3975,48.5865],[-75.339,48.384],[-75.321,48.24],[-75.3165,48.0375],[-75.31191,47.81241],[-75.30345,47.6424],[-75.2175,47.6055],[-74.9925,47.5245],[-74.8395,47.4525],[-74.7585,47.3895],[-74.691,47.3085],[-74.637,47.2095],[-74.5965,47.106],[-74.59425,47.01357],[-74.62125,46.88307],[-74.6775,46.7145],[-74.718,46.62],[-74.718,46.584],[-74.7135,46.5075],[-74.7045,46.3905],[-74.6685,46.269],[-74.6055,46.143],[-74.4975,45.963],[-74.3445,45.729],[-74.2455,45.5625],[-74.2005,45.4635],[-74.03562,45.33741],[-73.82871,45.23733],[-73.6326,45.20007]
 ]}},
 {type:'Feature',properties:{uuid:'d7ee3ac1-62a4-4881-810c-39fd5df3a25e',feature_class:'Primary Road'},geometry:{type:'LineString',coordinates:[[-76.36662,51.34419],[-76.2615,51.372],[-76.0365,51.48],[-75.6765,51.6105],[-75.1815,51.7635],[-74.826,51.8985],[-74.61,52.0155],[-74.385,52.1055],[-74.151,52.1685],[-74.01681,52.20108]]}}
]};
const graph=buildTransportGraph(fc);
assert.ok(graph.nodes.length>45);
assert.ok(graph.edges.length>45);
const luskan=nearestGraphNode(graph,{lon_fria:-76.36662,lat:51.34419},100);
const neverwinter=nearestGraphNode(graph,{lon_fria:-75.91491,lat:49.30596},100);
const waterdeep=nearestGraphNode(graph,{lon_fria:-73.6326,lat:45.20007},100);
assert.ok(luskan&&neverwinter&&waterdeep);
const ln=shortestPath(graph,luskan.node_id,neverwinter.node_id);
const nw=shortestPath(graph,neverwinter.node_id,waterdeep.node_id);
assert.ok(ln?.distance_m>200_000 && ln.distance_m<400_000);
assert.ok(nw?.distance_m>450_000 && nw.distance_m<650_000);
console.log(JSON.stringify({nodes:graph.nodes.length,edges:graph.edges.length,luskan_neverwinter_km:(ln.distance_m/1000).toFixed(1),neverwinter_waterdeep_km:(nw.distance_m/1000).toFixed(1)},null,2));
console.log('SPRINT13B_PATHWAYS: PASS');
