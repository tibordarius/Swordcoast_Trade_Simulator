export function displayToImagePoint({clientX,clientY,left,top,displayWidth,displayHeight,naturalWidth,naturalHeight}){
  if(displayWidth<=0||displayHeight<=0||naturalWidth<=0||naturalHeight<=0) throw new Error('invalid_dimensions');
  const x=(clientX-left)*(naturalWidth/displayWidth);
  const y=(clientY-top)*(naturalHeight/displayHeight);
  return {x:Math.max(0,Math.min(naturalWidth,x)),y:Math.max(0,Math.min(naturalHeight,y))};
}
export function normalizePoint({x,y},width,height){return {u:x/width,v:y/height};}
export function nominalDistanceKm(a,b,kmPerPx){return Math.hypot(a.x-b.x,a.y-b.y)*kmPerPx;}
export function makeControlPoint(name,p,width,height){
  const n=normalizePoint(p,width,height);
  return {name,x_px:Number(p.x.toFixed(2)),y_px:Number(p.y.toFixed(2)),u:Number(n.u.toFixed(8)),v:Number(n.v.toFixed(8))};
}
