use serde::{Deserialize, Serialize};

use crate::{
    CommodityId, InventoryAccountId, MarketId, MarketTradeId, MoneyAccountId, MoneyCp, Quantity,
    SimTick, UnitPrice, WorldState,
};

const PPM: i128 = 1_000_000;
const MAX_RATIO_PPM: i128 = 4_000_000;
const MAX_IMPACT_PPM: i128 = 750_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketSide {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarketListing {
    market_id: MarketId,
    commodity_id: CommodityId,
    inventory_account: InventoryAccountId,
    money_account: MoneyAccountId,
    reference_price: UnitPrice,
    target_stock: Quantity,
    depth: Quantity,
    spread_bps: u32,
    demand_window_ticks: u64,
}

impl MarketListing {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        market_id: MarketId,
        commodity_id: CommodityId,
        inventory_account: InventoryAccountId,
        money_account: MoneyAccountId,
        reference_price: UnitPrice,
        target_stock: Quantity,
        depth: Quantity,
        spread_bps: u32,
        demand_window_ticks: u64,
    ) -> Self {
        Self {
            market_id,
            commodity_id,
            inventory_account,
            money_account,
            reference_price,
            target_stock,
            depth,
            spread_bps,
            demand_window_ticks,
        }
    }

    #[must_use]
    pub fn market_id(&self) -> &MarketId {
        &self.market_id
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.commodity_id
    }

    #[must_use]
    pub fn inventory_account(&self) -> &InventoryAccountId {
        &self.inventory_account
    }

    #[must_use]
    pub fn money_account(&self) -> &MoneyAccountId {
        &self.money_account
    }

    #[must_use]
    pub const fn reference_price(&self) -> UnitPrice {
        self.reference_price
    }

    #[must_use]
    pub const fn target_stock(&self) -> Quantity {
        self.target_stock
    }

    #[must_use]
    pub const fn depth(&self) -> Quantity {
        self.depth
    }

    #[must_use]
    pub const fn spread_bps(&self) -> u32 {
        self.spread_bps
    }

    #[must_use]
    pub const fn demand_window_ticks(&self) -> u64 {
        self.demand_window_ticks
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PriceExplanation {
    pub stock: Quantity,
    pub target_stock: Quantity,
    pub stock_ratio_ppm: i64,
    pub scarcity_factor_ppm: i64,
    pub recent_requested: Quantity,
    pub recent_unmet: Quantity,
    pub demand_pressure_ppm: i64,
    pub reference_price: UnitPrice,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarketQuote {
    pub market_id: MarketId,
    pub commodity_id: CommodityId,
    pub tick: SimTick,
    pub fundamental: UnitPrice,
    pub bid: UnitPrice,
    pub ask: UnitPrice,
    pub explanation: PriceExplanation,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarketTrade {
    trade_id: MarketTradeId,
    tick: SimTick,
    market_id: MarketId,
    commodity_id: CommodityId,
    side: MarketSide,
    quantity: Quantity,
    average_unit_price: UnitPrice,
    total_value: MoneyCp,
}

impl MarketTrade {
    #[must_use]
    pub fn new(
        trade_id: MarketTradeId,
        tick: SimTick,
        market_id: MarketId,
        commodity_id: CommodityId,
        side: MarketSide,
        quantity: Quantity,
        average_unit_price: UnitPrice,
        total_value: MoneyCp,
    ) -> Self {
        Self {
            trade_id,
            tick,
            market_id,
            commodity_id,
            side,
            quantity,
            average_unit_price,
            total_value,
        }
    }

    #[must_use]
    pub fn trade_id(&self) -> &MarketTradeId {
        &self.trade_id
    }

    #[must_use]
    pub const fn tick(&self) -> SimTick {
        self.tick
    }

    #[must_use]
    pub fn market_id(&self) -> &MarketId {
        &self.market_id
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.commodity_id
    }

    #[must_use]
    pub const fn side(&self) -> MarketSide {
        self.side
    }

    #[must_use]
    pub const fn quantity(&self) -> Quantity {
        self.quantity
    }

    #[must_use]
    pub const fn average_unit_price(&self) -> UnitPrice {
        self.average_unit_price
    }

    #[must_use]
    pub const fn total_value(&self) -> MoneyCp {
        self.total_value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarketMathError {
    MissingInventoryAccount(InventoryAccountId),
    InvalidConfiguration,
    InvalidQuantity,
    ArithmeticOverflow,
    InvalidPrice,
}

pub fn derive_market_quote(
    state: &WorldState,
    listing: &MarketListing,
) -> Result<MarketQuote, MarketMathError> {
    if !listing.reference_price().is_positive()
        || listing.target_stock().get() <= 0
        || listing.depth().get() <= 0
        || listing.spread_bps() == 0
        || listing.spread_bps() > 10_000
        || listing.demand_window_ticks() == 0
    {
        return Err(MarketMathError::InvalidConfiguration);
    }

    let stock = state
        .inventory_balance(listing.inventory_account(), listing.commodity_id())
        .ok_or_else(|| MarketMathError::MissingInventoryAccount(listing.inventory_account().clone()))?;
    let stock_nonnegative = i128::from(stock.get().max(0));
    let target = i128::from(listing.target_stock().get());

    let ratio_numerator = stock_nonnegative
        .checked_mul(PPM)
        .ok_or(MarketMathError::ArithmeticOverflow)?;
    let stock_ratio_ppm_i128 = (ratio_numerator / target).min(MAX_RATIO_PPM);
    let stock_ratio_ppm =
        i64::try_from(stock_ratio_ppm_i128).map_err(|_| MarketMathError::ArithmeticOverflow)?;

    let scarcity_factor_i128 = if stock_ratio_ppm_i128 >= PPM {
        let excess = stock_ratio_ppm_i128 - PPM;
        let reduction = excess
            .checked_mul(400_000)
            .ok_or(MarketMathError::ArithmeticOverflow)?
            / PPM;
        PPM - reduction.min(500_000)
    } else {
        let shortage = PPM - stock_ratio_ppm_i128;
        let increase = shortage
            .checked_mul(1_200_000)
            .ok_or(MarketMathError::ArithmeticOverflow)?
            / PPM;
        PPM + increase
    };

    let scarcity_factor_ppm =
        i64::try_from(scarcity_factor_i128).map_err(|_| MarketMathError::ArithmeticOverflow)?;

    let window_start = SimTick::new(
        state
            .tick()
            .get()
            .saturating_sub(listing.demand_window_ticks()),
    );

    let mut recent_requested = 0_i64;
    let mut recent_unmet = 0_i64;

    for record in state.consumption_records() {
        if record.tick() < window_start {
            continue;
        }

        let Some(cohort) = state.population_cohort(record.cohort_id()) else {
            continue;
        };

        if cohort.inventory_account() != listing.inventory_account()
            || cohort.commodity_id() != listing.commodity_id()
        {
            continue;
        }

        recent_requested = recent_requested
            .checked_add(record.requested().get())
            .ok_or(MarketMathError::ArithmeticOverflow)?;
        recent_unmet = recent_unmet
            .checked_add(record.unmet().get())
            .ok_or(MarketMathError::ArithmeticOverflow)?;
    }

    let demand_pressure_i128 = if recent_requested > 0 {
        let unmet_share = (i128::from(recent_unmet)
            .checked_mul(PPM)
            .ok_or(MarketMathError::ArithmeticOverflow)?
            / i128::from(recent_requested))
        .min(PPM);

        PPM + unmet_share
            .checked_mul(300_000)
            .ok_or(MarketMathError::ArithmeticOverflow)?
            / PPM
    } else {
        PPM
    };

    let demand_pressure_ppm =
        i64::try_from(demand_pressure_i128).map_err(|_| MarketMathError::ArithmeticOverflow)?;

    let scarcity_price = listing
        .reference_price()
        .checked_mul_ppm(scarcity_factor_ppm)
        .map_err(|_| MarketMathError::ArithmeticOverflow)?;
    let fundamental = clamp_positive_price(
        scarcity_price
            .checked_mul_ppm(demand_pressure_ppm)
            .map_err(|_| MarketMathError::ArithmeticOverflow)?,
    )?;

    let half_spread_scaled = fundamental
        .scaled_value()
        .checked_mul(i128::from(listing.spread_bps()))
        .ok_or(MarketMathError::ArithmeticOverflow)?
        / 20_000;
    let half_spread_scaled = half_spread_scaled.max(1);

    let bid_scaled = fundamental
        .scaled_value()
        .checked_sub(half_spread_scaled)
        .ok_or(MarketMathError::ArithmeticOverflow)?
        .max(1);
    let ask_scaled = fundamental
        .scaled_value()
        .checked_add(half_spread_scaled)
        .ok_or(MarketMathError::ArithmeticOverflow)?;

    let bid = UnitPrice::new(bid_scaled, fundamental.scale())
        .map_err(|_| MarketMathError::InvalidPrice)?;
    let ask = UnitPrice::new(ask_scaled, fundamental.scale())
        .map_err(|_| MarketMathError::InvalidPrice)?;

    Ok(MarketQuote {
        market_id: listing.market_id().clone(),
        commodity_id: listing.commodity_id().clone(),
        tick: state.tick(),
        fundamental,
        bid,
        ask,
        explanation: PriceExplanation {
            stock,
            target_stock: listing.target_stock(),
            stock_ratio_ppm,
            scarcity_factor_ppm,
            recent_requested: Quantity::new(recent_requested),
            recent_unmet: Quantity::new(recent_unmet),
            demand_pressure_ppm,
            reference_price: listing.reference_price(),
        },
    })
}

pub fn execution_price(
    quote: &MarketQuote,
    side: MarketSide,
    quantity: Quantity,
    depth: Quantity,
) -> Result<UnitPrice, MarketMathError> {
    if quantity.get() <= 0 || depth.get() <= 0 {
        return Err(MarketMathError::InvalidQuantity);
    }

    let impact_ppm = i128::from(quantity.get())
        .checked_mul(250_000)
        .ok_or(MarketMathError::ArithmeticOverflow)?
        / i128::from(depth.get());
    let impact_ppm = impact_ppm.min(MAX_IMPACT_PPM);

    let factor_ppm = match side {
        MarketSide::Buy => PPM
            .checked_add(impact_ppm)
            .ok_or(MarketMathError::ArithmeticOverflow)?,
        MarketSide::Sell => PPM
            .checked_sub(impact_ppm)
            .ok_or(MarketMathError::ArithmeticOverflow)?,
    };

    let factor_ppm =
        i64::try_from(factor_ppm).map_err(|_| MarketMathError::ArithmeticOverflow)?;
    let base = match side {
        MarketSide::Buy => quote.ask,
        MarketSide::Sell => quote.bid,
    };

    clamp_positive_price(
        base.checked_mul_ppm(factor_ppm)
            .map_err(|_| MarketMathError::ArithmeticOverflow)?,
    )
}

fn clamp_positive_price(price: UnitPrice) -> Result<UnitPrice, MarketMathError> {
    if price.scaled_value() > 0 {
        return Ok(price);
    }

    UnitPrice::new(1, price.scale()).map_err(|_| MarketMathError::InvalidPrice)
}
