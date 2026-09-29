import test from 'node:test';import assert from 'node:assert/strict';import {displayToImagePoint,normalizePoint,nominalDistanceKm,makeControlPoint} from './calibration.mjs';
test('display center maps to configured image center',()=>{const p=displayToImagePoint({clientX:550,clientY:850,left:50,top:100,displayWidth:1000,displayHeight:1500,naturalWidth:6600,naturalHeight:10200});assert.equal(p.x,3300);assert.equal(p.y,5100);});
test('normalization is resolution independent',()=>{assert.deepEqual(normalizePoint({x:3300,y:5100},6600,10200),{u:.5,v:.5});});
test('vault nominal scale is preserved',()=>{assert.equal(nominalDistanceKm({x:0,y:0},{x:100,y:0},.275),27.500000000000004);});
test('control point export is bounded and normalized',()=>{assert.deepEqual(makeControlPoint('Waterdeep',{x:1650,y:2550},6600,10200),{name:'Waterdeep',x_px:1650,y_px:2550,u:.25,v:.25});});
