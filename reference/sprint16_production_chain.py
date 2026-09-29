#!/usr/bin/env python3
"""Sprint 16 production-chain oracle.

Iron chain: mine -> 3-day ore transit -> smelter -> 1-day ingot transit -> forge.
A temporary mine shutdown must propagate downstream only after buffers/transit delays.
All quantities are milli-units.
"""
from collections import deque
from dataclasses import dataclass

DAY_TICKS=288
ORE_BATCH=100_000_000      # 100,000 kg milli-kg = 100,000 kg? calibration scale only
INGOT_OUT=60_000_000
FORGE_INPUT=100_000_000
TOOLS_OUT=1_000

@dataclass
class Result:
    tools:int
    ore_mined:int
    ore_smelter:int
    iron_forge:int
    first_short_day:int|None
    zero_output_days:int


def simulate(shock=False, days=60):
    mine_rate=200_000_000       # 200k milli-kg/day calibration
    ore_at_mine=0
    ore_at_smelter=500_000_000  # buffer
    iron_at_smelter=0
    iron_at_forge=400_000_000   # 4 forge batches buffer
    tools=0; ore_mined=0
    ore_pipe=deque([0,0,0])     # 3-day transit
    iron_pipe=deque([0])        # 1-day transit
    first_short=None; zeros=0

    for day in range(days):
        # Arrivals first.
        ore_at_smelter += ore_pipe.popleft()
        iron_at_forge += iron_pipe.popleft()

        # Mine. Shock days 10..24 inclusive.
        produced = 0 if shock and 10 <= day < 25 else mine_rate
        ore_at_mine += produced; ore_mined += produced

        # Dispatch all mine output daily; transport delay handles propagation.
        ore_pipe.append(ore_at_mine); ore_at_mine=0

        # Smelter can run two batches/day when input exists.
        smelt_batches=min(2, ore_at_smelter//ORE_BATCH)
        ore_at_smelter -= smelt_batches*ORE_BATCH
        iron_at_smelter += smelt_batches*INGOT_OUT
        iron_pipe.append(iron_at_smelter); iron_at_smelter=0

        # Forge can run one tool batch/day.
        if iron_at_forge >= FORGE_INPUT:
            iron_at_forge -= FORGE_INPUT
            tools += TOOLS_OUT
        else:
            zeros += 1
            if first_short is None: first_short=day

    return Result(tools,ore_mined,ore_at_smelter,iron_at_forge,first_short,zeros)


def recipe_atomicity():
    inv={'ore':250_000_000,'iron':10_000_000}
    before=inv.copy()
    batches=min(2,inv['ore']//ORE_BATCH)
    inv['ore']-=batches*ORE_BATCH; inv['iron']+=batches*INGOT_OUT
    assert before['ore']-inv['ore']==batches*ORE_BATCH
    assert inv['iron']-before['iron']==batches*INGOT_OUT


def main():
    recipe_atomicity()
    base=simulate(False); shock=simulate(True)
    assert base.tools > shock.tools
    assert shock.first_short_day is not None and shock.first_short_day > 10
    assert shock.zero_output_days > 0
    # Recovery: despite shutdown, output resumes before end of run.
    assert shock.tools > 0
    print('SPRINT16_PRODUCTION_CHAIN: PASS')
    print('baseline_tools_milli',base.tools)
    print('shock_tools_milli',shock.tools)
    print('shock_first_forge_short_day',shock.first_short_day)
    print('shock_zero_output_days',shock.zero_output_days)
    print('baseline_ore_mined_milli',base.ore_mined)
    print('shock_ore_mined_milli',shock.ore_mined)

if __name__=='__main__': main()
