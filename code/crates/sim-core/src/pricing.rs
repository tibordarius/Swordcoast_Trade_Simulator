pub const BPS: i64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceExplanation {
    pub reserve_bps: i64,
    pub unmet_bps: i64,
    pub incoming_bps: i64,
    pub risk_bps: i64,
    pub total_multiplier_bps: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceState {
    pub fundamental_mcp: i64,
    pub bid_mcp: i64,
    pub ask_mcp: i64,
    pub explanation: PriceExplanation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketExecution {
    pub average_price_mcp: i64,
    pub terminal_impact_bps: i64,
    pub total_mcp: i64,
}

fn round_div(numerator: i64, denominator: i64) -> i64 {
    assert!(numerator >= 0 && denominator > 0);
    (numerator + denominator / 2) / denominator
}

#[allow(clippy::too_many_arguments)]
pub fn compute_price(
    reference_mcp: i64,
    available: i64,
    target: i64,
    recent_unmet: i64,
    daily_demand: i64,
    effective_incoming: i64,
    risk_bps: i64,
    liquidity_tier: u8,
) -> PriceState {
    assert!(reference_mcp > 0 && available >= 0 && target > 0);
    assert!(recent_unmet >= 0 && daily_demand > 0 && effective_incoming >= 0);

    let gap_bps = ((target - available) * BPS) / target;
    let reserve_component = if gap_bps >= 0 {
        (gap_bps * 15_000) / BPS
    } else {
        (gap_bps * 5_000) / BPS
    };

    let unmet_ratio_bps = ((recent_unmet * BPS) / daily_demand).min(20_000);
    let unmet_component = (unmet_ratio_bps * 5_000) / BPS;

    let incoming_ratio_bps = ((effective_incoming * BPS) / target).min(20_000);
    let incoming_component = -((incoming_ratio_bps * 3_000) / BPS).min(3_000);

    let total = (BPS + reserve_component + unmet_component + incoming_component + risk_bps)
        .clamp(4_000, 50_000);
    let fundamental = round_div(reference_mcp * total, BPS).max(1);

    let spread_bps = match liquidity_tier.clamp(1, 5) {
        5 => 50,
        4 => 80,
        3 => 120,
        2 => 200,
        _ => 350,
    };
    let half = spread_bps / 2;
    let bid = round_div(fundamental * (BPS - half), BPS).max(1);
    let ask = round_div(fundamental * (BPS + half), BPS).max(bid);

    PriceState {
        fundamental_mcp: fundamental,
        bid_mcp: bid,
        ask_mcp: ask,
        explanation: PriceExplanation {
            reserve_bps: reserve_component,
            unmet_bps: unmet_component,
            incoming_bps: incoming_component,
            risk_bps,
            total_multiplier_bps: total,
        },
    }
}

pub fn market_buy(quantity_milli: i64, ask_mcp: i64, depth_milli: i64) -> MarketExecution {
    assert!(quantity_milli > 0 && ask_mcp > 0 && depth_milli > 0);
    let terminal_impact_bps = ((quantity_milli * 200) / depth_milli).min(5_000);
    let average_impact_bps = terminal_impact_bps / 2;
    let average_price_mcp = round_div(ask_mcp * (BPS + average_impact_bps), BPS);
    MarketExecution {
        average_price_mcp,
        terminal_impact_bps,
        total_mcp: (average_price_mcp * quantity_milli + 500) / 1_000,
    }
}

pub fn market_sell(quantity_milli: i64, bid_mcp: i64, depth_milli: i64) -> MarketExecution {
    assert!(quantity_milli > 0 && bid_mcp > 0 && depth_milli > 0);
    let terminal_impact_bps = ((quantity_milli * 200) / depth_milli).min(5_000);
    let average_impact_bps = terminal_impact_bps / 2;
    let average_price_mcp = round_div(bid_mcp * (BPS - average_impact_bps), BPS);
    MarketExecution {
        average_price_mcp,
        terminal_impact_bps,
        total_mcp: (average_price_mcp * quantity_milli + 500) / 1_000,
    }
}
