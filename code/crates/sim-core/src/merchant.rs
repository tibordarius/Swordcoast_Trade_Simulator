use crate::{compute_price, market_buy, market_sell, CalibratedRoute, CargoProfile, PriceState, BPS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketView {
    pub available_milli: i64,
    pub target_milli: i64,
    pub daily_demand_milli: i64,
    pub liquidity_tier: u8,
    pub depth_milli: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradeRouteEconomics {
    pub variable_cost_mcp_per_base_unit: i64,
    pub fixed_cost_mcp: i64,
    pub risk_loss_bps: i64,
    pub capacity_milli: i64,
}

impl TradeRouteEconomics {
    pub fn cost_mcp(&self, quantity_milli: i64) -> i64 {
        self.fixed_cost_mcp + (quantity_milli * self.variable_cost_mcp_per_base_unit) / 1_000
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opportunity {
    pub quantity_milli: i64,
    pub purchase_mcp: i64,
    pub expected_revenue_mcp: i64,
    pub transport_mcp: i64,
    pub expected_loss_mcp: i64,
    pub expected_profit_mcp: i64,
    pub roi_bps: i64,
    pub accepted: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn evaluate_opportunity(
    origin: MarketView,
    destination: MarketView,
    route: TradeRouteEconomics,
    quantity_milli: i64,
    capital_mcp: i64,
    min_roi_bps: i64,
    reference_price_mcp: i64,
) -> Opportunity {
    let origin_price = compute_price(
        reference_price_mcp,
        origin.available_milli,
        origin.target_milli,
        0,
        origin.daily_demand_milli,
        0,
        0,
        origin.liquidity_tier,
    );
    let destination_price = compute_price(
        reference_price_mcp,
        destination.available_milli,
        destination.target_milli,
        0,
        destination.daily_demand_milli,
        0,
        0,
        destination.liquidity_tier,
    );

    let buy = market_buy(quantity_milli, origin_price.ask_mcp, origin.depth_milli);
    let sell = market_sell(quantity_milli, destination_price.bid_mcp, destination.depth_milli);
    let transport = route.cost_mcp(quantity_milli);
    let expected_loss = (buy.total_mcp * route.risk_loss_bps) / BPS;
    let deployed = buy.total_mcp + transport;
    let profit = sell.total_mcp - buy.total_mcp - transport - expected_loss;
    let roi_bps = if deployed > 0 {
        (profit * BPS) / deployed
    } else {
        i64::MIN / 2
    };

    let accepted = quantity_milli > 0
        && quantity_milli <= route.capacity_milli
        && deployed <= capital_mcp
        && profit > 0
        && roi_bps >= min_roi_bps;

    Opportunity {
        quantity_milli,
        purchase_mcp: buy.total_mcp,
        expected_revenue_mcp: sell.total_mcp,
        transport_mcp: transport,
        expected_loss_mcp: expected_loss,
        expected_profit_mcp: profit,
        roi_bps,
        accepted,
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutableOpportunity {
    pub quantity_milli: i64,
    pub purchase_mcp: i64,
    pub expected_revenue_mcp: i64,
    pub freight_mcp: i64,
    pub expected_loss_mcp: i64,
    pub expected_profit_mcp: i64,
    pub roi_bps: i64,
    pub accepted: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn evaluate_executable_opportunity(
    origin_quote: PriceState,
    origin_depth_milli: i64,
    destination_quote: PriceState,
    destination_depth_milli: i64,
    route: &CalibratedRoute,
    cargo: CargoProfile,
    quantity_milli: i64,
    capital_mcp: i64,
    min_roi_bps: i64,
) -> ExecutableOpportunity {
    if quantity_milli <= 0 || origin_depth_milli <= 0 || destination_depth_milli <= 0 {
        return ExecutableOpportunity {
            quantity_milli,
            purchase_mcp: 0,
            expected_revenue_mcp: 0,
            freight_mcp: 0,
            expected_loss_mcp: 0,
            expected_profit_mcp: i64::MIN / 4,
            roi_bps: i64::MIN / 4,
            accepted: false,
        };
    }

    let buy = market_buy(quantity_milli, origin_quote.ask_mcp, origin_depth_milli);
    let sell = market_sell(
        quantity_milli,
        destination_quote.bid_mcp,
        destination_depth_milli,
    );
    let freight = route.freight_cost_mcp(quantity_milli, cargo);
    let expected_loss = (buy.total_mcp * route.risk_bps) / BPS;
    let deployed = buy
        .total_mcp
        .checked_add(freight)
        .expect("merchant deployed capital overflow");
    let profit = sell.total_mcp - buy.total_mcp - freight - expected_loss;
    let roi_bps = if deployed > 0 {
        (profit * BPS) / deployed
    } else {
        i64::MIN / 2
    };
    let accepted = route.accepts_quantity(quantity_milli, cargo)
        && deployed <= capital_mcp
        && profit > 0
        && roi_bps >= min_roi_bps;

    ExecutableOpportunity {
        quantity_milli,
        purchase_mcp: buy.total_mcp,
        expected_revenue_mcp: sell.total_mcp,
        freight_mcp: freight,
        expected_loss_mcp: expected_loss,
        expected_profit_mcp: profit,
        roi_bps,
        accepted,
    }
}
