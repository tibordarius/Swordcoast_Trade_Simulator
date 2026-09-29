#!/usr/bin/env python3
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SEED = ROOT / "seed"

def load(name):
    with open(SEED / name, "r", encoding="utf-8") as f:
        return json.load(f)

def unique(records, key, label):
    vals = [r[key] for r in records]
    if len(vals) != len(set(vals)):
        raise SystemExit(f"duplicate {label}: {vals}")

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

    market_ids = {m["id"] for m in markets}
    commodity_ids = {c["id"] for c in commodities}

    expected_markets = set(world["mvp_markets"])
    if expected_markets != market_ids:
        raise SystemExit(f"world market mismatch: world={expected_markets}, files={market_ids}")

    if len(markets) != 6:
        raise SystemExit(f"expected 6 MVP markets, got {len(markets)}")
    if len(commodities) != 24:
        raise SystemExit(f"expected 24 MVP commodities, got {len(commodities)}")
    if "CMD-GRAIN" not in commodity_ids:
        raise SystemExit("grain benchmark commodity missing")

    allowed_provenance = {"fr_canon", "campaign_canon", "inference", "simulation_generated"}
    for r in [*markets, *routes, *commodities, *market_states]:
        if r["provenance"] not in allowed_provenance:
            raise SystemExit(f"invalid provenance on {r.get('id', r.get('commodity_id'))}: {r['provenance']}")

    for c in commodities:
        if not isinstance(c["quantity_scale"], int) or c["quantity_scale"] <= 0:
            raise SystemExit(f"invalid quantity scale: {c['id']}")
        if c["reference_price_cp"] is not None and (
            not isinstance(c["reference_price_cp"], int) or c["reference_price_cp"] < 0
        ):
            raise SystemExit(f"reference price must be integer cp: {c['id']}")

    for r in routes:
        if r["from"] not in market_ids or r["to"] not in market_ids:
            raise SystemExit(f"route endpoint missing: {r['id']}")
        if r["from"] == r["to"]:
            raise SystemExit(f"self route: {r['id']}")

    state_keys = set()
    for state in market_states:
        key = (state["market_id"], state["commodity_id"])
        if key in state_keys:
            raise SystemExit(f"duplicate market state: {key}")
        state_keys.add(key)
        if state["market_id"] not in market_ids:
            raise SystemExit(f"unknown market in state: {key}")
        if state["commodity_id"] not in commodity_ids:
            raise SystemExit(f"unknown commodity in state: {key}")
        for field in (
            "on_hand_milli", "target_reserve_milli", "reference_price_mcp",
            "daily_supply_milli", "daily_demand_milli", "depth_milli",
            "incoming_committed_milli", "risk_bps",
        ):
            if not isinstance(state[field], int):
                raise SystemExit(f"{field} must be integer fixed-point: {key}")
        if state["on_hand_milli"] < 0 or state["target_reserve_milli"] <= 0:
            raise SystemExit(f"invalid inventory state: {key}")
        if state["reference_price_mcp"] <= 0 or state["daily_demand_milli"] <= 0:
            raise SystemExit(f"invalid price/demand state: {key}")
        if not 1 <= state["liquidity_tier"] <= 5:
            raise SystemExit(f"invalid liquidity tier: {key}")
        if not 0 <= state["risk_bps"] <= 10000:
            raise SystemExit(f"invalid risk bps: {key}")

    expected_state_keys = {(m, c) for m in market_ids for c in commodity_ids}
    if state_keys != expected_state_keys:
        missing = sorted(expected_state_keys - state_keys)
        extra = sorted(state_keys - expected_state_keys)
        raise SystemExit(f"market state matrix mismatch: missing={missing[:10]} extra={extra[:10]}")

    print("seed validation: PASS")
    print(
        f"markets={len(markets)} commodities={len(commodities)} routes={len(routes)} "
        f"market_states={len(market_states)}"
    )
    print(f"tick_minutes={world['tick_minutes']} world_seed={world['world_seed']}")

if __name__ == "__main__":
    main()
