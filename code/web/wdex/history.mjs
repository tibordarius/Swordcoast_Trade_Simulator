export const HISTORY_INTERVALS=Object.freeze({
  '1H':12,
  '1D':288,
  '10D':2880,
  '30D':8640
});

function bi(v){return typeof v==='bigint'?v:BigInt(v);}
function out(v){return bi(v).toString();}

/** Aggregate authoritative market observations into exact OHLCV candles.
 *  Monetary/quantity values stay BigInt; no browser Number arithmetic touches them.
 */
export function aggregateCandles(rows, intervalTicks){
  const n=BigInt(intervalTicks);
  if(n<=0n) throw new Error('interval_ticks_must_be_positive');
  const buckets=new Map();
  for(const row of rows){
    const tick=bi(row.tick);
    const start=(tick/n)*n;
    const key=start.toString();
    const price=bi(row.last_mcp);
    const volume=bi(row.volume_milli??0);
    const inv=bi(row.inventory_milli??0);
    const imports=bi(row.imports_milli??0);
    const exports=bi(row.exports_milli??0);
    let b=buckets.get(key);
    if(!b){
      b={bucket_start_tick:start,first_tick:tick,last_tick:tick,open:price,high:price,low:price,close:price,volume:0n,inventory_close:inv,imports:0n,exports:0n};
      buckets.set(key,b);
    }
    // Allow unsorted input while preserving OHLC by authoritative tick.
    if(tick<b.first_tick){b.first_tick=tick;b.open=price;}
    if(tick>b.last_tick){b.last_tick=tick;b.close=price;b.inventory_close=inv;}
    else if(tick===b.last_tick){b.close=price;b.inventory_close=inv;}
    if(price>b.high)b.high=price;
    if(price<b.low)b.low=price;
    b.volume+=volume;b.imports+=imports;b.exports+=exports;
  }
  return [...buckets.values()].sort((a,b)=>a.bucket_start_tick<b.bucket_start_tick?-1:1).map(b=>({
    bucket_start_tick:out(b.bucket_start_tick),
    open_mcp:out(b.open),high_mcp:out(b.high),low_mcp:out(b.low),close_mcp:out(b.close),
    volume_milli:out(b.volume),inventory_close_milli:out(b.inventory_close),
    imports_milli:out(b.imports),exports_milli:out(b.exports)
  }));
}

export function intervalTicks(label){
  const v=HISTORY_INTERVALS[label];
  if(!v) throw new Error(`unsupported_interval:${label}`);
  return v;
}
