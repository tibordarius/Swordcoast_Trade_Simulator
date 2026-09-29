import assert from 'node:assert/strict';
import {svgPointToFria,friaToMythDrannor,mythDrannorToFria,vincentyDistanceMeters,pointAlongPolyline,TORIL} from './toril-gcs.mjs';

const wd=svgPointToFria(1181.86,497.777);
assert.ok(Math.abs(wd.lat-45.20007)<1e-6);
assert.ok(Math.abs(wd.lon_fria-(-73.6326))<1e-6);
assert.ok(Math.abs(friaToMythDrannor(wd.lon_fria)-(-20.15109851201))<1e-9);
assert.ok(Math.abs(mythDrannorToFria(friaToMythDrannor(wd.lon_fria))-wd.lon_fria)<1e-9);
assert.equal(TORIL.inv_f,160.25);

const nw=svgPointToFria(1156.501,452.156);
const d=vincentyDistanceMeters(wd,nw);
assert.ok(d>450_000 && d<500_000,`unexpected Waterdeep-Neverwinter distance ${d}`);
assert.equal(vincentyDistanceMeters(wd,wd),0);

const mid=pointAlongPolyline([wd,nw],0.5);
assert.ok(mid.lat>wd.lat && mid.lat<nw.lat);
assert.ok(mid.lon_fria<wd.lon_fria && mid.lon_fria>nw.lon_fria);
assert.deepEqual(pointAlongPolyline([wd,nw],0),wd);
assert.deepEqual(pointAlongPolyline([wd,nw],1),nw);
console.log(JSON.stringify({status:'PASS',waterdeep:wd,waterdeep_md:friaToMythDrannor(wd.lon_fria),waterdeep_neverwinter_km:Number((d/1000).toFixed(3))}));
