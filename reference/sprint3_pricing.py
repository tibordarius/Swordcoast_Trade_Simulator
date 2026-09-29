#!/usr/bin/env python3
"""Sprint 3 pricing reference model.

Price quotes use milli-copper pieces (mcp): 1000 mcp = 1 cp.
All pressure components are integer basis points (10,000 bps = 1.0x baseline).
"""
from dataclasses import dataclass

BPS = 10_000

@dataclass(frozen=True)
class PriceExplanation:
    reserve_bps: int
    unmet_bps: int
    incoming_bps: int
    risk_bps: int
    total_multiplier_bps: int

@dataclass(frozen=True)
class PriceState:
    fundamental_mcp: int
    bid_mcp: int
    ask_mcp: int
    explanation: PriceExplanation

def clamp(v, lo, hi):
    return max(lo, min(hi, v))

def round_div(n: int, d: int) -> int:
    assert d > 0 and n >= 0
    return (n + d // 2) // d

def compute_price(*, reference_mcp: int, available: int, target: int, recent_unmet: int, daily_demand: int, effective_incoming: int = 0, risk_bps: int = 0, liquidity_tier: int = 4) -> PriceState:
    assert reference_mcp > 0 and available >= 0 and target > 0
    assert recent_unmet >= 0 and daily_demand > 0 and effective_incoming >= 0

    gap_bps = ((target - available) * BPS) // target
    if gap_bps >= 0:
        reserve_component = (gap_bps * 15_000) // BPS
    else:
        reserve_component = (gap_bps * 5_000) // BPS

    unmet_ratio_bps = min(20_000, (recent_unmet * BPS) // daily_demand)
    unmet_component = (unmet_ratio_bps * 5_000) // BPS

    incoming_ratio_bps = min(20_000, (effective_incoming * BPS) // target)
    incoming_component = -min(3_000, (incoming_ratio_bps * 3_000) // BPS)

    total = clamp(BPS + reserve_component + unmet_component + incoming_component + risk_bps, 4_000, 50_000)
    fundamental = max(1, round_div(reference_mcp * total, BPS))

    spread_by_tier = {5: 50, 4: 80, 3: 120, 2: 200, 1: 350}
    spread_bps = spread_by_tier[clamp(liquidity_tier, 1, 5)]
    half = spread_bps // 2
    bid = max(1, round_div(fundamental * (BPS - half), BPS))
    ask = max(bid, round_div(fundamental * (BPS + half), BPS))

    return PriceState(fundamental, bid, ask, PriceExplanation(reserve_component, unmet_component, incoming_component, risk_bps, total))

def market_buy(*, quantity_milli: int, ask_mcp: int, depth_milli: int):
    assert quantity_milli > 0 and ask_mcp > 0 and depth_milli > 0
    terminal_impact_bps = min(5_000, (quantity_milli * 200) // depth_milli)
    average_impact_bps = terminal_impact_bps // 2
    avg_price_mcp = round_div(ask_mcp * (BPS + average_impact_bps), BPS)
    total_mcp = round_div(avg_price_mcp * quantity_milli, 1000)
    return {
        "average_price_mcp": avg_price_mcp,
        "terminal_impact_bps": terminal_impact_bps,
        "total_mcp": total_mcp,
    }

def market_sell(*, quantity_milli: int, bid_mcp: int, depth_milli: int):
    assert quantity_milli > 0 and bid_mcp > 0 and depth_milli > 0
    terminal_impact_bps = min(5_000, (quantity_milli * 200) // depth_milli)
    average_impact_bps = terminal_impact_bps // 2
    avg_price_mcp = round_div(bid_mcp * (BPS - average_impact_bps), BPS)
    total_mcp = round_div(avg_price_mcp * quantity_milli, 1000)
    return {
        "average_price_mcp": avg_price_mcp,
        "terminal_impact_bps": terminal_impact_bps,
        "total_mcp": total_mcp,
    }

def main():
    normal = compute_price(reference_mcp=2000, available=700_000_000, target=700_000_000, recent_unmet=0, daily_demand=25_000_000, liquidity_tier=4)
    surplus = compute_price(reference_mcp=2000, available=1_400_000_000, target=700_000_000, recent_unmet=0, daily_demand=25_000_000, liquidity_tier=4)
    shortage = compute_price(reference_mcp=2000, available=350_000_000, target=700_000_000, recent_unmet=0, daily_demand=25_000_000, liquidity_tier=4)
    severe = compute_price(reference_mcp=2000, available=50_000_000, target=700_000_000, recent_unmet=12_500_000, daily_demand=25_000_000, effective_incoming=100_000_000, risk_bps=300, liquidity_tier=4)

    assert surplus.fundamental_mcp < normal.fundamental_mcp < shortage.fundamental_mcp < severe.fundamental_mcp
    assert normal.fundamental_mcp == 2000
    assert surplus.fundamental_mcp == 1000
    assert shortage.fundamental_mcp == 3500

    small = market_buy(quantity_milli=10_000_000, ask_mcp=normal.ask_mcp, depth_milli=40_000_000)
    large = market_buy(quantity_milli=100_000_000, ask_mcp=normal.ask_mcp, depth_milli=40_000_000)
    assert large["average_price_mcp"] > small["average_price_mcp"] >= normal.ask_mcp

    incoming_none = compute_price(reference_mcp=2000, available=350_000_000, target=700_000_000, recent_unmet=0, daily_demand=25_000_000, effective_incoming=0)
    incoming_big = compute_price(reference_mcp=2000, available=350_000_000, target=700_000_000, recent_unmet=0, daily_demand=25_000_000, effective_incoming=350_000_000)
    assert incoming_big.fundamental_mcp < incoming_none.fundamental_mcp

    for label, state in [("surplus", surplus), ("normal", normal), ("shortage", shortage), ("severe", severe)]:
        print(label, {
            "fundamental_mcp": state.fundamental_mcp,
            "bid_mcp": state.bid_mcp,
            "ask_mcp": state.ask_mcp,
            "components": state.explanation,
        })
    print("small buy", small)
    print("large buy", large)
    print("price monotonicity: PASS")
    print("incoming supply expectation: PASS")
    print("market slippage: PASS")
    print("integer pricing: PASS")

if __name__ == "__main__":
    main()
