export const TORIL = Object.freeze({
  a_m: 6_410_000,
  b_m: 6_370_000,
  inv_f: 160.25,
  myth_drannor_prime_meridian_fria_deg: -53.48150148799,
  svg_world: Object.freeze({width:4000,height:2000,west:-180,east:180,south:-90,north:90})
});

const toRad = d => d * Math.PI / 180;
const toDeg = r => r * 180 / Math.PI;

export function normalizeLon(lon){
  let x = Number(lon);
  while(x > 180) x -= 360;
  while(x < -180) x += 360;
  return x;
}

export function svgPointToFria(x,y,meta=TORIL.svg_world){
  const lon = meta.west + (Number(x)/meta.width) * (meta.east-meta.west);
  const lat = meta.north - (Number(y)/meta.height) * (meta.north-meta.south);
  return {lon_fria:lon,lat};
}

export function friaToMythDrannor(lonFria){
  return normalizeLon(Number(lonFria) - TORIL.myth_drannor_prime_meridian_fria_deg);
}

export function mythDrannorToFria(lonMd){
  return normalizeLon(Number(lonMd) + TORIL.myth_drannor_prime_meridian_fria_deg);
}

// Vincenty inverse solution on Toril's ellipsoid. Returns integer-friendly metric distance.
export function vincentyDistanceMeters(a,b){
  const lat1=toRad(a.lat), lat2=toRad(b.lat);
  const L=toRad(normalizeLon(b.lon_fria-a.lon_fria));
  const A=TORIL.a_m, B=TORIL.b_m, f=(A-B)/A;
  const U1=Math.atan((1-f)*Math.tan(lat1));
  const U2=Math.atan((1-f)*Math.tan(lat2));
  const sinU1=Math.sin(U1), cosU1=Math.cos(U1), sinU2=Math.sin(U2), cosU2=Math.cos(U2);
  let lambda=L, prev;
  let sinSigma=0,cosSigma=0,sigma=0,sinAlpha=0,cosSqAlpha=0,cos2SigmaM=0;
  for(let i=0;i<200;i++){
    const sinL=Math.sin(lambda), cosL=Math.cos(lambda);
    const x=cosU2*sinL;
    const y=cosU1*sinU2-sinU1*cosU2*cosL;
    sinSigma=Math.hypot(x,y);
    if(sinSigma===0) return 0;
    cosSigma=sinU1*sinU2+cosU1*cosU2*cosL;
    sigma=Math.atan2(sinSigma,cosSigma);
    sinAlpha=cosU1*cosU2*sinL/sinSigma;
    cosSqAlpha=1-sinAlpha*sinAlpha;
    cos2SigmaM=cosSqAlpha===0?0:cosSigma-2*sinU1*sinU2/cosSqAlpha;
    const C=f/16*cosSqAlpha*(4+f*(4-3*cosSqAlpha));
    prev=lambda;
    lambda=L+(1-C)*f*sinAlpha*(sigma+C*sinSigma*(cos2SigmaM+C*cosSigma*(-1+2*cos2SigmaM*cos2SigmaM)));
    if(Math.abs(lambda-prev)<1e-12) break;
    if(i===199) throw new Error('vincenty_no_convergence');
  }
  const uSq=cosSqAlpha*(A*A-B*B)/(B*B);
  const bigA=1+uSq/16384*(4096+uSq*(-768+uSq*(320-175*uSq)));
  const bigB=uSq/1024*(256+uSq*(-128+uSq*(74-47*uSq)));
  const deltaSigma=bigB*sinSigma*(cos2SigmaM+bigB/4*(cosSigma*(-1+2*cos2SigmaM*cos2SigmaM)-bigB/6*cos2SigmaM*(-3+4*sinSigma*sinSigma)*(-3+4*cos2SigmaM*cos2SigmaM)));
  return B*bigA*(sigma-deltaSigma);
}

export function polylineDistanceMeters(coords){
  let total=0;
  for(let i=1;i<coords.length;i++) total+=vincentyDistanceMeters(coords[i-1],coords[i]);
  return total;
}

function shortestLonDelta(a,b){return normalizeLon(b-a);}

export function interpolateSegment(a,b,t){
  if(t<=0) return {...a};
  if(t>=1) return {...b};
  const dlon=shortestLonDelta(a.lon_fria,b.lon_fria);
  return {lon_fria:normalizeLon(a.lon_fria+dlon*t),lat:a.lat+(b.lat-a.lat)*t};
}

export function pointAlongPolyline(coords,progress){
  if(!coords.length) throw new Error('empty_polyline');
  if(coords.length===1) return {...coords[0]};
  const p=Math.max(0,Math.min(1,Number(progress)));
  if(p===0) return {...coords[0]};
  if(p===1) return {...coords.at(-1)};
  const seg=[];let total=0;
  for(let i=1;i<coords.length;i++){const d=vincentyDistanceMeters(coords[i-1],coords[i]);seg.push(d);total+=d;}
  let target=total*p,acc=0;
  for(let i=0;i<seg.length;i++){
    if(acc+seg[i]>=target){const t=seg[i]===0?0:(target-acc)/seg[i];return interpolateSegment(coords[i],coords[i+1],t);}
    acc+=seg[i];
  }
  return {...coords.at(-1)};
}

export function toMapLibreLngLat(p){return [p.lon_fria,p.lat];}
export function displayCoordinates(p){return {lat:p.lat,lon_md:friaToMythDrannor(p.lon_fria)};}
