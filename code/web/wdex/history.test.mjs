import test from 'node:test';
import assert from 'node:assert/strict';
import {aggregateCandles,intervalTicks} from './history.mjs';

test('one-hour candles use 12 five-minute ticks',()=>assert.equal(intervalTicks('1H'),12));

test('OHLC follows tick order even when input is unsorted',()=>{
 const rows=[
  {tick:'2',last_mcp:'2200',volume_milli:'5',inventory_milli:'98'},
  {tick:'0',last_mcp:'2000',volume_milli:'7',inventory_milli:'100'},
  {tick:'1',last_mcp:'1800',volume_milli:'11',inventory_milli:'99'},
  {tick:'11',last_mcp:'2100',volume_milli:'13',inventory_milli:'90'}
 ];
 const [c]=aggregateCandles(rows,12);
 assert.deepEqual(c,{bucket_start_tick:'0',open_mcp:'2000',high_mcp:'2200',low_mcp:'1800',close_mcp:'2100',volume_milli:'36',inventory_close_milli:'90',imports_milli:'0',exports_milli:'0'});
});

test('large fixed-point history remains exact beyond Number safe integer',()=>{
 const huge='9007199254740993123';
 const [c]=aggregateCandles([{tick:'0',last_mcp:huge,volume_milli:huge,inventory_milli:huge}],12);
 assert.equal(c.close_mcp,huge);assert.equal(c.volume_milli,huge);
});

test('buckets aggregate imports and exports',()=>{
 const c=aggregateCandles([
  {tick:'0',last_mcp:'1000',volume_milli:'0',inventory_milli:'10',imports_milli:'4',exports_milli:'1'},
  {tick:'12',last_mcp:'1100',volume_milli:'0',inventory_milli:'11',imports_milli:'2',exports_milli:'3'}
 ],12);
 assert.equal(c.length,2);assert.equal(c[0].imports_milli,'4');assert.equal(c[1].exports_milli,'3');
});
