use sim_core::{evaluate_opportunity, MarketView, TradeRouteEconomics};

fn athkatla(stock: i64) -> MarketView {
    MarketView {
        available_milli: stock,
        target_milli: 700_000_000,
        daily_demand_milli: 25_000_000,
        liquidity_tier: 4,
        depth_milli: 300_000_000,
    }
}

fn waterdeep(stock: i64) -> MarketView {
    MarketView {
        available_milli: stock,
        target_milli: 1_200_000_000,
        daily_demand_milli: 45_000_000,
        liquidity_tier: 5,
        depth_milli: 250_000_000,
    }
}

fn route() -> TradeRouteEconomics {
    TradeRouteEconomics {
        variable_cost_mcp_per_base_unit: 300,
        fixed_cost_mcp: 1_500_000,
        risk_loss_bps: 100,
        capacity_milli: 500_000_000,
    }
}

#[test]
fn forward_arbitrage_is_selected_and_reverse_is_rejected() {
    let forward = evaluate_opportunity(
        athkatla(950_000_000),
        waterdeep(640_000_000),
        route(),
        100_000_000,
        500_000_000,
        500,
        2_000,
    );
    let reverse = evaluate_opportunity(
        waterdeep(640_000_000),
        athkatla(950_000_000),
        route(),
        100_000_000,
        500_000_000,
        500,
        2_000,
    );
    assert!(forward.accepted);
    assert_eq!(forward.expected_profit_mcp, 139_145_000);
    assert_eq!(forward.roi_bps, 7_063);
    assert!(!reverse.accepted);
    assert!(reverse.expected_profit_mcp < 0);
}

#[test]
fn repeated_completed_arbitrage_converges() {
    let mut origin = 950_000_000;
    let mut destination = 640_000_000;
    let mut capital = 500_000_000;
    let quantity = 100_000_000;
    let mut accepted = 0;
    let mut first_roi = None;
    let final_roi;

    loop {
        let opp = evaluate_opportunity(
            athkatla(origin),
            waterdeep(destination),
            route(),
            quantity,
            capital,
            500,
            2_000,
        );
        if first_roi.is_none() {
            first_roi = Some(opp.roi_bps);
        }
        if !opp.accepted {
            final_roi = opp.roi_bps;
            break;
        }
        accepted += 1;
        origin -= quantity;
        destination += quantity;
        capital += opp.expected_profit_mcp;
        assert!(accepted < 20);
    }

    assert_eq!(accepted, 3);
    assert_eq!(final_roi, 254);
    assert!(final_roi < first_roi.unwrap());
}
