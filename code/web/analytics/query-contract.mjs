/** Safe query contract between the WDEX UI and DuckDB-Wasm worker.
 *  The UI sends parameters, never arbitrary SQL.
 */
export const ANALYTIC_QUERY_TYPES=Object.freeze([
  'price_history','market_spread','market_volume','volatility'
]);

export function validateAnalyticsRequest(r){
  if(!r || !ANALYTIC_QUERY_TYPES.includes(r.type)) throw new Error('unsupported_analytics_query');
  if(typeof r.branch_id!=='string'||!r.branch_id) throw new Error('branch_id_required');
  if(r.commodity_id!==undefined && (typeof r.commodity_id!=='string'||!r.commodity_id)) throw new Error('bad_commodity_id');
  const min=BigInt(r.min_tick??0), max=BigInt(r.max_tick??min);
  if(min<0n||max<min) throw new Error('bad_tick_range');
  return {...r,min_tick:min.toString(),max_tick:max.toString()};
}
