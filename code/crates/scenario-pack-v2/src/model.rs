use serde::{Deserialize, Serialize};
use sim_kernel_v2::{
    CommodityId, InventoryAccountId, InventoryAccountKind, MarketId, MoneyAccountId,
    MoneyAccountKind, PlaceId, PlaceKind, RouteEdgeId, ScenarioPackId, SimTick, UnitId,
};

pub const SCENARIO_PACK_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceStatus {
    Source,
    ReviewedMapping,
    ScenarioAssumption,
    Derived,
    Generated,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackManifest {
    pub schema_version: u32,
    pub pack_id: ScenarioPackId,
    pub revision: u32,
    pub campaign_epoch: String,
    pub world_seed: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnitSpec {
    pub id: UnitId,
    pub name: String,
    pub dimension: String,
    pub base_unit: UnitId,
    pub numerator: u64,
    pub denominator: u64,
    pub status: ProvenanceStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommoditySpec {
    pub id: CommodityId,
    pub name: String,
    pub base_unit: UnitId,
    pub mass_grams_per_base_unit: u64,
    pub volume_cm3_per_base_unit: u64,
    pub status: ProvenanceStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlaceSpec {
    pub id: PlaceId,
    pub name: String,
    pub kind: PlaceKind,
    pub status: ProvenanceStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarketSpec {
    pub id: MarketId,
    pub name: String,
    pub place_id: PlaceId,
    pub status: ProvenanceStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteSpec {
    pub id: RouteEdgeId,
    pub from_market: MarketId,
    pub to_market: MarketId,
    pub bidirectional: bool,
    pub distance_meters: u64,
    pub base_travel_ticks: SimTick,
    pub status: ProvenanceStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryAccountSpec {
    pub id: InventoryAccountId,
    pub kind: InventoryAccountKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoneyAccountSpec {
    pub id: MoneyAccountId,
    pub kind: MoneyAccountKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpeningInventorySpec {
    pub account_id: InventoryAccountId,
    pub commodity_id: CommodityId,
    pub quantity: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpeningMoneySpec {
    pub account_id: MoneyAccountId,
    pub amount_cp: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScenarioPack {
    pub manifest: PackManifest,
    pub units: Vec<UnitSpec>,
    pub commodities: Vec<CommoditySpec>,
    pub places: Vec<PlaceSpec>,
    pub markets: Vec<MarketSpec>,
    pub routes: Vec<RouteSpec>,
    pub inventory_accounts: Vec<InventoryAccountSpec>,
    pub money_accounts: Vec<MoneyAccountSpec>,
    pub opening_inventory: Vec<OpeningInventorySpec>,
    pub opening_money: Vec<OpeningMoneySpec>,
}
