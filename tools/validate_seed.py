#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SEED = ROOT / "seed"


def load(name):
    with open(SEED / name, "r", encoding="utf-8") as file:
        return json.load(file)


def unique(records, key, label):
    values = [record[key] for record in records]
    if len(values) != len(set(values)):
        raise SystemExit(f"duplicate {label}: {values}")


def main():
    world = load("world_v0.json")
    markets = load("markets_v0.json")["markets"]
    commodities = load("commodities_v0.json")["commodities"]
    routes = load("routes_v0.json")["routes"]
    market_states = load("market_states_v0.json")["states"]

    unique(markets, "id", "market id")
    unique(markets, "exchange", "exchange code")
    unique(commodities, "id", "commodity id")
    unique(routes, "id", "route id")

    state_keys = [(state["market_id"], state["commodity_id"]) for state in market_states]
    if len(state_keys) != len(set(state_keys)):
        raise SystemExit(f"duplicate market state: {state_keys}")

    market_ids = {market["id"] for market in markets}
    commodity_ids = {commodity["id"] for commodity in commodities}

    expected_markets = set(world["mvp_markets"])
    if expected_markets != market_ids:
        raise SystemExit(
            f"world market mismatch: world={expected_markets}, files={market_ids}"
        )

    if len(markets) != 6:
        raise SystemExit(f"expected 6 MVP markets, got {len(markets)}")
    if len(commodities) != 24:
        raise SystemExit(f"expected 24 MVP commodities, got {len(commodities)}")
    if "CMD-GRAIN" not in commodity_ids:
        raise SystemExit("grain benchmark commodity missing")

    allowed_provenance = {
        "fr_canon",
        "campaign_canon",
        "inference",
        "simulation_generated",
    }
    for record in [*markets, *routes]:
        if record["provenance"] not in allowed_provenance:
            raise SystemExit(
                f"invalid provenance on {record['id']}: {record['provenance']}"
            )

    for commodity in commodities:
        if (
            not isinstance(commodity["quantity_scale"], int)
            or commodity["quantity_scale"] <= 0
        ):
            raise SystemExit(f"invalid quantity scale: {commodity['id']}")
        price = commodity["reference_price_cp"]
        if price is not None and (
            not isinstance(price, int) or price < 0
        ):
            raise SystemExit(
                f"reference price must be integer cp: {commodity['id']}"
            )

    for route in routes:
        if route["from"] not in market_ids or route["to"] not in market_ids:
            raise SystemExit(f"route endpoint missing: {route['id']}")
        if route["from"] == route["to"]:
            raise SystemExit(f"self route: {route['id']}")

    for state in market_states:
        market_id = state["market_id"]
        commodity_id = state["commodity_id"]
        if market_id not in market_ids:
            raise SystemExit(
                f"market state references unknown market: {market_id}"
            )
        if commodity_id not in commodity_ids:
            raise SystemExit(
                f"market state references unknown commodity: {commodity_id}"
            )
        for key in (
            "on_hand_milli",
            "daily_supply_milli",
            "incoming_committed_milli",
            "risk_bps",
        ):
            if not isinstance(state[key], int) or state[key] < 0:
                raise SystemExit(
                    f"invalid {key} on {market_id}/{commodity_id}: {state[key]}"
                )
        for key in (
            "target_reserve_milli",
            "reference_price_mcp",
            "daily_demand_milli",
            "depth_milli",
        ):
            if not isinstance(state[key], int) or state[key] <= 0:
                raise SystemExit(
                    f"invalid {key} on {market_id}/{commodity_id}: {state[key]}"
                )
        if not 1 <= state["liquidity_tier"] <= 5:
            raise SystemExit(
                f"invalid liquidity tier on {market_id}/{commodity_id}"
            )
        if state["risk_bps"] > 10_000:
            raise SystemExit(
                f"risk bps exceeds 100% on {market_id}/{commodity_id}"
            )

    print("seed validation: PASS")
    print(
        f"markets={len(markets)} commodities={len(commodities)} "
        f"routes={len(routes)} market_states={len(market_states)}"
    )
    print(
        f"tick_minutes={world['tick_minutes']} world_seed={world['world_seed']}"
    )


if __name__ == "__main__":
    main()
