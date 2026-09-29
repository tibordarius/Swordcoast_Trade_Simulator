use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use sim_core::{
    CalibratedRoute, CargoProfile, MarketCommodityKey, MarketCommodityState, WorldState,
};

const ALLOWED_PROVENANCE: [&str; 4] = [
    "fr_canon",
    "campaign_canon",
    "inference",
    "simulation_generated",
];

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
struct RouteFile {
    routes: Vec<RouteDefinition>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MarketDefinition {
    pub id: String,
    pub name: String,
    pub exchange: String,
    pub liquidity_tier: u8,
    pub role: String,
    pub provenance: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CommodityDefinition {
    pub id: String,
    pub name: String,
    pub base_unit: String,
    pub quantity_scale: i64,
    pub family: String,
    pub perishability: String,
    pub fungibility: String,
    pub reference_price_cp: Option<i64>,
    pub reference_price_status: String,
    pub provenance: String,
    #[serde(default)]
    pub mass_grams_per_base_unit: i64,
    #[serde(default)]
    pub volume_cm3_per_base_unit: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RouteDefinition {
    pub id: String,
    pub from: String,
    pub to: String,
    pub mode: String,
    pub bidirectional: bool,
    pub distance_miles: i64,
    pub travel_ticks: u64,
    pub risk_bps: i64,
    pub freight_mcp_per_kg: i64,
    pub capacity_kg: i64,
    pub capacity_m3: i64,
    pub provenance: String,
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
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedBundle {
    markets: BTreeMap<String, MarketDefinition>,
    commodities: BTreeMap<String, CommodityDefinition>,
    routes: BTreeMap<String, RouteDefinition>,
    market_states: Vec<MarketStateSeed>,
}

impl SeedBundle {
    pub fn embedded_mvp() -> Result<Self, String> {
        Self::from_json_with_routes(
            include_str!("../../../../seed/markets_v0.json"),
            include_str!("../../../../seed/commodities_v0.json"),
            include_str!("../../../../seed/market_states_v0.json"),
            include_str!("../../../../seed/routes_v0.json"),
        )
    }

    pub fn from_json(
        markets_json: &str,
        commodities_json: &str,
        market_states_json: &str,
    ) -> Result<Self, String> {
        Self::from_json_with_routes(
            markets_json,
            commodities_json,
            market_states_json,
            r#"{"routes":[]}"#,
        )
    }

    pub fn from_json_with_routes(
        markets_json: &str,
        commodities_json: &str,
        market_states_json: &str,
        routes_json: &str,
    ) -> Result<Self, String> {
        let markets_file: MarketFile =
            serde_json::from_str(markets_json).map_err(|error| format!("markets seed: {error}"))?;
        let commodities_file: CommodityFile = serde_json::from_str(commodities_json)
            .map_err(|error| format!("commodities seed: {error}"))?;
        let states_file: MarketStateFile = serde_json::from_str(market_states_json)
            .map_err(|error| format!("market states seed: {error}"))?;
        let routes_file: RouteFile =
            serde_json::from_str(routes_json).map_err(|error| format!("routes seed: {error}"))?;

        let markets = collect_markets(markets_file.markets)?;
        let commodities = collect_commodities(commodities_file.commodities)?;
        let routes = collect_routes(routes_file.routes, &markets)?;
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
                return Err(format!("duplicate market state {} / {}", key.0, key.1));
            }
        }

        let expected: BTreeSet<_> = markets
            .keys()
            .flat_map(|market_id| {
                commodities
                    .keys()
                    .map(move |commodity_id| (market_id.clone(), commodity_id.clone()))
            })
            .collect();
        if state_keys != expected {
            let missing: Vec<_> = expected.difference(&state_keys).take(8).cloned().collect();
            let extra: Vec<_> = state_keys.difference(&expected).take(8).cloned().collect();
            return Err(format!(
                "market state matrix incomplete: missing={missing:?} extra={extra:?}"
            ));
        }

        market_states.sort_by(|a, b| {
            (&a.market_id, &a.commodity_id).cmp(&(&b.market_id, &b.commodity_id))
        });

        Ok(Self {
            markets,
            commodities,
            routes,
            market_states,
        })
    }

    pub fn instantiate_world(&self, seed: u64) -> WorldState {
        let mut world = WorldState::new(seed);
        for commodity in self.commodities.values() {
            if commodity.mass_grams_per_base_unit > 0 && commodity.volume_cm3_per_base_unit > 0 {
                world.insert_cargo_profile(
                    &commodity.id,
                    CargoProfile::new(
                        commodity.mass_grams_per_base_unit,
                        commodity.volume_cm3_per_base_unit,
                    ),
                );
            }
        }
        for route in self.routes.values() {
            world.insert_route(CalibratedRoute::new(
                &route.id,
                &route.from,
                &route.to,
                route.bidirectional,
                route.travel_ticks,
                route.risk_bps,
                route.freight_mcp_per_kg,
                route.capacity_kg,
                route.capacity_m3,
            ));
        }
        for state in &self.market_states {
            world.insert_market(
                MarketCommodityKey::new(&state.market_id, &state.commodity_id),
                MarketCommodityState::new(
                    state.on_hand_milli,
                    state.target_reserve_milli,
                    state.reference_price_mcp,
                    state.daily_supply_milli,
                    state.daily_demand_milli,
                    state.liquidity_tier,
                    state.depth_milli,
                    state.incoming_committed_milli,
                    state.risk_bps,
                ),
            );
        }
        world
    }

    pub fn market(&self, market_id: &str) -> Option<&MarketDefinition> {
        self.markets.get(market_id)
    }

    pub fn commodity(&self, commodity_id: &str) -> Option<&CommodityDefinition> {
        self.commodities.get(commodity_id)
    }

    pub fn route(&self, route_id: &str) -> Option<&RouteDefinition> {
        self.routes.get(route_id)
    }

    pub fn markets_len(&self) -> usize {
        self.markets.len()
    }

    pub fn commodities_len(&self) -> usize {
        self.commodities.len()
    }

    pub fn routes_len(&self) -> usize {
        self.routes.len()
    }

    pub fn market_states(&self) -> &[MarketStateSeed] {
        &self.market_states
    }

    pub fn states_for_market<'a>(
        &'a self,
        market_id: &'a str,
    ) -> impl Iterator<Item = &'a MarketStateSeed> + 'a {
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
        validate_provenance(&definition.provenance, &definition.id)?;
        if !(1..=5).contains(&definition.liquidity_tier) {
            return Err(format!("invalid liquidity tier for market {}", definition.id));
        }
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
        validate_provenance(&definition.provenance, &definition.id)?;
        if definition.quantity_scale <= 0 {
            return Err(format!("invalid quantity scale for {}", definition.id));
        }
        let dimensions_present =
            definition.mass_grams_per_base_unit > 0 && definition.volume_cm3_per_base_unit > 0;
        let dimensions_absent =
            definition.mass_grams_per_base_unit == 0 && definition.volume_cm3_per_base_unit == 0;
        if !dimensions_present && !dimensions_absent {
            return Err(format!(
                "commodity logistics dimensions must both be positive or both omitted: {}",
                definition.id
            ));
        }
        let id = definition.id.clone();
        if out.insert(id.clone(), definition).is_some() {
            return Err(format!("duplicate commodity id {id}"));
        }
    }
    Ok(out)
}

fn collect_routes(
    definitions: Vec<RouteDefinition>,
    markets: &BTreeMap<String, MarketDefinition>,
) -> Result<BTreeMap<String, RouteDefinition>, String> {
    let mut out = BTreeMap::new();
    for definition in definitions {
        validate_provenance(&definition.provenance, &definition.id)?;
        if definition.from == definition.to
            || !markets.contains_key(&definition.from)
            || !markets.contains_key(&definition.to)
        {
            return Err(format!("invalid route endpoints for {}", definition.id));
        }
        if definition.mode != "sea"
            || definition.distance_miles <= 0
            || definition.travel_ticks == 0
            || !(0..=10_000).contains(&definition.risk_bps)
            || definition.freight_mcp_per_kg < 0
            || definition.capacity_kg <= 0
            || definition.capacity_m3 <= 0
        {
            return Err(format!("invalid calibrated route {}", definition.id));
        }
        let id = definition.id.clone();
        if out.insert(id.clone(), definition).is_some() {
            return Err(format!("duplicate route id {id}"));
        }
    }
    Ok(out)
}

fn validate_state(state: &MarketStateSeed) -> Result<(), String> {
    validate_provenance(
        &state.provenance,
        &format!("{} / {}", state.market_id, state.commodity_id),
    )?;
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

fn validate_provenance(value: &str, entity: &str) -> Result<(), String> {
    if ALLOWED_PROVENANCE.contains(&value) {
        Ok(())
    } else {
        Err(format!("invalid provenance {value} on {entity}"))
    }
}
