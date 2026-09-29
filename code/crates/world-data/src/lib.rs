use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct MarketFile {
    markets: Vec<MarketDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
struct CommodityFile {
    commodities: Vec<CommodityDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
struct MarketStateFile {
    states: Vec<MarketStateSeed>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketDefinition {
    pub id: String,
    pub name: String,
    pub exchange: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommodityDefinition {
    pub id: String,
    pub name: String,
    pub base_unit: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MarketStateSeed {
    pub market_id: String,
    pub commodity_id: String,
    pub on_hand_milli: i64,
    pub target_reserve_milli: i64,
    pub reference_price_mcp: i64,
    pub daily_supply_milli: i64,
    pub daily_demand_milli: i64,
    pub liquidity_tier: u8,
    pub depth_milli: i64,
    pub incoming_committed_milli: i64,
    pub risk_bps: i64,
}

#[derive(Debug, Clone)]
pub struct SeedBundle {
    markets: BTreeMap<String, MarketDefinition>,
    commodities: BTreeMap<String, CommodityDefinition>,
    market_states: Vec<MarketStateSeed>,
}

impl SeedBundle {
    pub fn embedded_mvp() -> Result<Self, String> {
        Self::from_json(
            include_str!("../../../../seed/markets_v0.json"),
            include_str!("../../../../seed/commodities_v0.json"),
            include_str!("../../../../seed/market_states_v0.json"),
        )
    }

    pub fn from_json(
        markets_json: &str,
        commodities_json: &str,
        market_states_json: &str,
    ) -> Result<Self, String> {
        let markets_file: MarketFile =
            serde_json::from_str(markets_json).map_err(|error| error.to_string())?;
        let commodities_file: CommodityFile =
            serde_json::from_str(commodities_json).map_err(|error| error.to_string())?;
        let states_file: MarketStateFile =
            serde_json::from_str(market_states_json).map_err(|error| error.to_string())?;

        let markets = collect_markets(markets_file.markets)?;
        let commodities = collect_commodities(commodities_file.commodities)?;
        let mut market_states = states_file.states;

        let mut state_keys = BTreeSet::new();
        for state in &market_states {
            validate_state(state)?;
            if !markets.contains_key(&state.market_id) {
                return Err(format!(
                    "market state references unknown market {}",
                    state.market_id
                ));
            }
            if !commodities.contains_key(&state.commodity_id) {
                return Err(format!(
                    "market state references unknown commodity {}",
                    state.commodity_id
                ));
            }
            let key = (state.market_id.clone(), state.commodity_id.clone());
            if !state_keys.insert(key.clone()) {
                return Err(format!(
                    "duplicate market state {} / {}",
                    key.0, key.1
                ));
            }
        }

        market_states.sort_by(|a, b| {
            (&a.market_id, &a.commodity_id).cmp(&(&b.market_id, &b.commodity_id))
        });

        Ok(Self {
            markets,
            commodities,
            market_states,
        })
    }

    pub fn market(&self, market_id: &str) -> Option<&MarketDefinition> {
        self.markets.get(market_id)
    }

    pub fn commodity(&self, commodity_id: &str) -> Option<&CommodityDefinition> {
        self.commodities.get(commodity_id)
    }

    pub fn markets_len(&self) -> usize {
        self.markets.len()
    }

    pub fn commodities_len(&self) -> usize {
        self.commodities.len()
    }

    pub fn market_states(&self) -> &[MarketStateSeed] {
        &self.market_states
    }

    pub fn states_for_market(
        &self,
        market_id: &str,
    ) -> impl Iterator<Item = &MarketStateSeed> {
        self.market_states
            .iter()
            .filter(move |state| state.market_id == market_id)
    }
}

fn collect_markets(
    definitions: Vec<MarketDefinition>,
) -> Result<BTreeMap<String, MarketDefinition>, String> {
    let mut out = BTreeMap::new();
    for definition in definitions {
        let id = definition.id.clone();
        if out.insert(id.clone(), definition).is_some() {
            return Err(format!("duplicate market id {id}"));
        }
    }
    Ok(out)
}

fn collect_commodities(
    definitions: Vec<CommodityDefinition>,
) -> Result<BTreeMap<String, CommodityDefinition>, String> {
    let mut out = BTreeMap::new();
    for definition in definitions {
        let id = definition.id.clone();
        if out.insert(id.clone(), definition).is_some() {
            return Err(format!("duplicate commodity id {id}"));
        }
    }
    Ok(out)
}

fn validate_state(state: &MarketStateSeed) -> Result<(), String> {
    if state.on_hand_milli < 0 {
        return Err(format!("negative on-hand for {}", state.commodity_id));
    }
    if state.target_reserve_milli <= 0 {
        return Err(format!("invalid reserve for {}", state.commodity_id));
    }
    if state.reference_price_mcp <= 0 {
        return Err(format!("invalid reference price for {}", state.commodity_id));
    }
    if state.daily_supply_milli < 0 || state.daily_demand_milli <= 0 {
        return Err(format!("invalid daily flow for {}", state.commodity_id));
    }
    if !(1..=5).contains(&state.liquidity_tier) {
        return Err(format!("invalid liquidity tier for {}", state.commodity_id));
    }
    if state.depth_milli <= 0 {
        return Err(format!("invalid depth for {}", state.commodity_id));
    }
    if state.incoming_committed_milli < 0 {
        return Err(format!("negative incoming stock for {}", state.commodity_id));
    }
    if !(0..=10_000).contains(&state.risk_bps) {
        return Err(format!("invalid risk bps for {}", state.commodity_id));
    }
    Ok(())
}
