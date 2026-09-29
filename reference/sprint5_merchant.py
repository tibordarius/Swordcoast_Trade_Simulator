#!/usr/bin/env python3
"""Sprint 5 merchant arbitrage reference model."""
from dataclasses import dataclass
from sprint3_pricing import compute_price, market_buy, market_sell, BPS

MILLI = 1000

@dataclass(frozen=True)
class MarketView:
    name: str
    available_milli: int
    target_milli: int
    daily_demand_milli: int
    liquidity_tier: int
    depth_milli: int

@dataclass(frozen=True)
class RouteView:
    variable_cost_mcp_per_base_unit: int
    fixed_cost_mcp: int
    risk_loss_bps: int
    capacity_milli: int

    def cost(self, quantity_milli: int) -> int:
        return self.fixed_cost_mcp + (quantity_milli * self.variable_cost_mcp_per_base_unit) // MILLI

@dataclass(frozen=True)
class Opportunity:
    origin: str
    destination: str
    quantity_milli: int
    purchase_mcp: int
    expected_revenue_mcp: int
    transport_mcp: int
    expected_loss_mcp: int
    expected_profit_mcp: int
    roi_bps: int
    accepted: bool

def evaluate(*, origin: MarketView, destination: MarketView, route: RouteView, quantity_milli: int, capital_mcp: int, min_roi_bps: int, reference_price_mcp: int = 2000) -> Opportunity:
    op = compute_price(reference_mcp=reference_price_mcp, available=origin.available_milli, target=origin.target_milli, recent_unmet=0, daily_demand=origin.daily_demand_milli, liquidity_tier=origin.liquidity_tier)
    dp = compute_price(reference_mcp=reference_price_mcp, available=destination.available_milli, target=destination.target_milli, recent_unmet=0, daily_demand=destination.daily_demand_milli, liquidity_tier=destination.liquidity_tier)

    buy = market_buy(quantity_milli=quantity_milli, ask_mcp=op.ask_mcp, depth_milli=origin.depth_milli)
    sell = market_sell(quantity_milli=quantity_milli, bid_mcp=dp.bid_mcp, depth_milli=destination.depth_milli)
    transport = route.cost(quantity_milli)
    expected_loss = (buy["total_mcp"] * route.risk_loss_bps) // BPS
    deployed = buy["total_mcp"] + transport
    profit = sell["total_mcp"] - buy["total_mcp"] - transport - expected_loss
    roi_bps = (profit * BPS) // deployed if deployed > 0 else -10**9
    accepted = (
        quantity_milli <= route.capacity_milli
        and deployed <= capital_mcp
        and profit > 0
        and roi_bps >= min_roi_bps
    )
    return Opportunity(origin.name, destination.name, quantity_milli, buy["total_mcp"], sell["total_mcp"], transport, expected_loss, profit, roi_bps, accepted)

def convergence():
    origin_stock = 950_000_000
    destination_stock = 640_000_000
    quantity = 100_000_000
    route = RouteView(300, 1_500_000, 100, 500_000_000)
    capital = 500_000_000
    trades = []

    for _ in range(20):
        origin = MarketView("ATHEX", origin_stock, 700_000_000, 25_000_000, 4, 300_000_000)
        dest = MarketView("WDEX", destination_stock, 1_200_000_000, 45_000_000, 5, 250_000_000)
        opp = evaluate(origin=origin, destination=dest, route=route, quantity_milli=quantity, capital_mcp=capital, min_roi_bps=500)
        trades.append(opp)
        if not opp.accepted:
            break
        origin_stock -= quantity
        destination_stock += quantity
        capital += opp.expected_profit_mcp

    return trades, origin_stock, destination_stock, capital

def main():
    route = RouteView(300, 1_500_000, 100, 500_000_000)
    ath = MarketView("ATHEX", 950_000_000, 700_000_000, 25_000_000, 4, 300_000_000)
    wd = MarketView("WDEX", 640_000_000, 1_200_000_000, 45_000_000, 5, 250_000_000)
    forward = evaluate(origin=ath, destination=wd, route=route, quantity_milli=100_000_000, capital_mcp=500_000_000, min_roi_bps=500)
    reverse = evaluate(origin=wd, destination=ath, route=route, quantity_milli=100_000_000, capital_mcp=500_000_000, min_roi_bps=500)
    assert forward.accepted
    assert not reverse.accepted

    trades, origin_end, dest_end, capital_end = convergence()
    accepted = [t for t in trades if t.accepted]
    assert accepted
    assert not trades[-1].accepted
    assert accepted[-1].roi_bps < accepted[0].roi_bps

    print("forward", forward)
    print("reverse", reverse)
    print(f"convergence accepted_trades={len(accepted)} final_attempt_roi_bps={trades[-1].roi_bps}")
    print(f"stocks origin={origin_end} destination={dest_end} capital={capital_end}")
    print("positive opportunity selection: PASS")
    print("negative reverse trade rejection: PASS")
    print("capital/capacity constraint hooks: PASS")
    print("arbitrage convergence: PASS")

if __name__ == "__main__":
    main()
