use crate::{compute_price, InventoryLedger, PriceState};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketCommodityKey {
    market_id: String,
    commodity_id: String,
}

impl MarketCommodityKey {
    pub fn new(market_id: impl Into<String>, commodity_id: impl Into<String>) -> Self {
        Self {
            market_id: market_id.into(),
            commodity_id: commodity_id.into(),
        }
    }

    pub fn market_id(&self) -> &str {
        &self.market_id
    }

    pub fn commodity_id(&self) -> &str {
        &self.commodity_id
    }

    pub(crate) fn append_stable_bytes(&self, bytes: &mut Vec<u8>) {
        append_string(bytes, &self.market_id);
        append_string(bytes, &self.commodity_id);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketCommodityState {
    inventory: InventoryLedger,
    reference_price_mcp: i64,
    daily_supply_milli: i64,
    daily_demand_milli: i64,
    liquidity_tier: u8,
    depth_milli: i64,
    incoming_committed_milli: i64,
    risk_bps: i64,
    recent_unmet_milli: i64,
    volume_milli: i64,
}

impl MarketCommodityState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        on_hand_milli: i64,
        target_reserve_milli: i64,
        reference_price_mcp: i64,
        daily_supply_milli: i64,
        daily_demand_milli: i64,
        liquidity_tier: u8,
        depth_milli: i64,
        incoming_committed_milli: i64,
        risk_bps: i64,
    ) -> Self {
        assert!(on_hand_milli >= 0);
        assert!(target_reserve_milli > 0);
        assert!(reference_price_mcp > 0);
        assert!(daily_supply_milli >= 0);
        assert!(daily_demand_milli > 0);
        assert!((1..=5).contains(&liquidity_tier));
        assert!(depth_milli > 0);
        assert!(incoming_committed_milli >= 0);

        Self {
            inventory: InventoryLedger::new(on_hand_milli, 0, target_reserve_milli),
            reference_price_mcp,
            daily_supply_milli,
            daily_demand_milli,
            liquidity_tier,
            depth_milli,
            incoming_committed_milli,
            risk_bps,
            recent_unmet_milli: 0,
            volume_milli: 0,
        }
    }

    pub fn quote(&self) -> PriceState {
        compute_price(
            self.reference_price_mcp,
            self.inventory.available(),
            self.inventory.target_reserve,
            self.recent_unmet_milli,
            self.daily_demand_milli,
            self.incoming_committed_milli,
            self.risk_bps,
            self.liquidity_tier,
        )
    }

    pub fn advance_hour(&mut self, hour_index: u64) {
        let supply = exact_hour_amount(self.daily_supply_milli, hour_index);
        let demand = exact_hour_amount(self.daily_demand_milli, hour_index);

        self.inventory.add_production(supply);
        let unmet_before = self.inventory.unmet_demand;
        let fulfilled = self.inventory.consume(demand);
        self.recent_unmet_milli = self.inventory.unmet_demand - unmet_before;
        self.volume_milli = self
            .volume_milli
            .checked_add(fulfilled)
            .expect("market volume overflow");
    }

    pub fn on_hand_milli(&self) -> i64 {
        self.inventory.on_hand
    }

    pub fn target_reserve_milli(&self) -> i64 {
        self.inventory.target_reserve
    }

    pub fn daily_supply_milli(&self) -> i64 {
        self.daily_supply_milli
    }

    pub fn daily_demand_milli(&self) -> i64 {
        self.daily_demand_milli
    }

    pub fn depth_milli(&self) -> i64 {
        self.depth_milli
    }

    pub fn liquidity_tier(&self) -> u8 {
        self.liquidity_tier
    }

    pub fn incoming_committed_milli(&self) -> i64 {
        self.incoming_committed_milli
    }

    pub fn recent_unmet_milli(&self) -> i64 {
        self.recent_unmet_milli
    }

    pub fn volume_milli(&self) -> i64 {
        self.volume_milli
    }

    pub(crate) fn append_stable_bytes(&self, bytes: &mut Vec<u8>) {
        for value in [
            self.inventory.on_hand,
            self.inventory.reserved,
            self.inventory.target_reserve,
            self.inventory.unmet_demand,
            self.inventory.produced,
            self.inventory.consumed,
            self.inventory.spoiled,
            self.reference_price_mcp,
            self.daily_supply_milli,
            self.daily_demand_milli,
            i64::from(self.liquidity_tier),
            self.depth_milli,
            self.incoming_committed_milli,
            self.risk_bps,
            self.recent_unmet_milli,
            self.volume_milli,
        ] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn exact_hour_amount(daily_milli: i64, hour_index: u64) -> i64 {
    assert!(hour_index > 0);
    let current = (i128::from(daily_milli) * i128::from(hour_index)) / 24;
    let previous = (i128::from(daily_milli) * i128::from(hour_index - 1)) / 24;
    i64::try_from(current - previous).expect("hourly market flow overflow")
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    let encoded = value.as_bytes();
    let len = u16::try_from(encoded.len()).expect("market key too long");
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(encoded);
}
