import {performance} from 'node:perf_hooks';
import {aggregateCandles} from '../code/web/wdex/history.mjs';
const rows=[];
for(let t=0;t<100000;t++) rows.push({tick:String(t),last_mcp:String(1800+(t%401)),volume_milli:String((t%17)*1000),inventory_milli:String(1_000_000_000-(t%10000)*1000),imports_milli:String(t%29===0?1000000:0),exports_milli:String(t%31===0?500000:0)});
const s=performance.now();const hourly=aggregateCandles(rows,12);const daily=aggregateCandles(rows,288);const ms=performance.now()-s;
if(hourly.length!==8334||daily.length!==348) throw new Error('unexpected_bucket_count');
console.log(JSON.stringify({status:'PASS',rows:rows.length,hourly:hourly.length,daily:daily.length,aggregate_ms:+ms.toFixed(2)}));
