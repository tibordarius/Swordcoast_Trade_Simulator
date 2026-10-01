use sim_kernel_v2::{
    CommodityDef, MarketDef, PlaceDef, RegistryError, RouteDef, ScenarioRegistry, UnitDef,
};

use crate::ValidatedScenarioPack;

pub fn compile_registry(pack: &ValidatedScenarioPack) -> Result<ScenarioRegistry, RegistryError> {
    let units = pack
        .pack()
        .units
        .iter()
        .map(|record| UnitDef {
            id: record.id.clone(),
            name: record.name.clone(),
            dimension: record.dimension.clone(),
            base_unit: record.base_unit.clone(),
            numerator: record.numerator,
            denominator: record.denominator,
        })
        .collect();

    let commodities = pack
        .pack()
        .commodities
        .iter()
        .map(|record| CommodityDef {
            id: record.id.clone(),
            name: record.name.clone(),
            base_unit: record.base_unit.clone(),
        })
        .collect();

    let places = pack
        .pack()
        .places
        .iter()
        .map(|record| PlaceDef {
            id: record.id.clone(),
            name: record.name.clone(),
        })
        .collect();

    let markets = pack
        .pack()
        .markets
        .iter()
        .map(|record| MarketDef {
            id: record.id.clone(),
            name: record.name.clone(),
            place_id: record.place_id.clone(),
        })
        .collect();

    let routes = pack
        .pack()
        .routes
        .iter()
        .map(|record| RouteDef {
            id: record.id.clone(),
            from_market: record.from_market.clone(),
            to_market: record.to_market.clone(),
            bidirectional: record.bidirectional,
            distance_meters: record.distance_meters,
        })
        .collect();

    ScenarioRegistry::new(units, commodities, places, markets, routes)
}
