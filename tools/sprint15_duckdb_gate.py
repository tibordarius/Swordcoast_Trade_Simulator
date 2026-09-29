#!/usr/bin/env python3
"""Native DuckDB/Parquet gate. Intended for CI where `duckdb` is installed."""
import shutil, tempfile
from pathlib import Path
try:
    import duckdb
except ImportError as e:
    raise SystemExit('DUCKDB_GATE_UNAVAILABLE: install duckdb in CI') from e


def main():
    root=Path(tempfile.mkdtemp(prefix='wdex-duckdb-'))
    try:
        con=duckdb.connect()
        con.execute('''CREATE TABLE market_tick_source(
            world_id VARCHAR, branch_id VARCHAR, tick BIGINT, market_id VARCHAR,
            commodity_id VARCHAR, bid_mcp BIGINT, ask_mcp BIGINT, last_mcp BIGINT,
            volume_milli BIGINT, inventory_milli BIGINT, imports_milli BIGINT,
            exports_milli BIGINT)''')
        rows=[]
        for t in range(24):
            wd=2000+t*4+(20 if t%5==0 else 0); bg=1800+t*3+(10 if t%7==0 else 0)
            rows += [
              ('WORLD','MAIN',t,'WDEX','GRAIN',wd-4,wd+4,wd,1000+t*10,1_000_000,0,0),
              ('WORLD','MAIN',t,'BGEX','GRAIN',bg-4,bg+4,bg,800+t*7,1_000_000,0,0)]
        con.executemany('INSERT INTO market_tick_source VALUES (?,?,?,?,?,?,?,?,?,?,?,?)',rows)
        target_dir=root/'history'/'market_ticks'
        target_dir.parent.mkdir(parents=True, exist_ok=True)
        target=target_dir.as_posix()
        con.execute(f'''COPY (
          SELECT *, floor(tick/105120)::BIGINT AS sim_year
          FROM market_tick_source ORDER BY market_id, commodity_id, tick
        ) TO '{target}' (FORMAT PARQUET, COMPRESSION ZSTD,
          PARTITION_BY(world_id,branch_id,sim_year), FILENAME_PATTERN 'data_{{uuid}}')''')
        glob=(root/'history'/'market_ticks'/'**'/'*.parquet').as_posix()
        count=con.execute(f"SELECT count(*) FROM read_parquet('{glob}', hive_partitioning=true)").fetchone()[0]
        assert count==48, count
        first=con.execute(f'''WITH h AS (
          SELECT tick,market_id,last_mcp FROM read_parquet('{glob}',hive_partitioning=true)
          WHERE commodity_id='GRAIN'), p AS (
          SELECT tick,max(last_mcp) FILTER(WHERE market_id='WDEX') wd,
          max(last_mcp) FILTER(WHERE market_id='BGEX') bg FROM h GROUP BY tick)
          SELECT wd-bg FROM p WHERE tick=0''').fetchone()[0]
        assert first==210, first
        # Confirm integer survives beyond JS safe integer through Parquet round-trip.
        con.execute("INSERT INTO market_tick_source VALUES ('WORLD','MAIN',105120,'WDEX','GEMS',9007199254740993,9007199254740995,9007199254740994,1,1,0,0)")
        exact=(root/'exact.parquet').as_posix()
        con.execute(f"COPY (SELECT * FROM market_tick_source WHERE commodity_id='GEMS') TO '{exact}' (FORMAT PARQUET)")
        x=con.execute(f"SELECT last_mcp FROM read_parquet('{exact}')").fetchone()[0]
        assert x==9007199254740994, x
        print('SPRINT15_DUCKDB_PARQUET_GATE: PASS')
        print('duckdb_version',duckdb.__version__)
        print('rows',count)
        print('first_spread_mcp',first)
        print('bigint_roundtrip',x)
    finally:
        shutil.rmtree(root,ignore_errors=True)
if __name__=='__main__': main()
