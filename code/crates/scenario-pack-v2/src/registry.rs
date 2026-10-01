use sim_kernel_v2::{
    CommodityDef, DataStatus, MarketDef, PlaceDef, RegistryError, RouteDef, ScenarioRegistry,
    UnitDef,
};

use crate::{ProvenanceStatus, ValidatedScenarioPack};

fn data_status(status: ProvenanceStatus) -> DataStatus {
    match status {
        ProvenanceStatus::Source => DataStatus::Source,
        ProvenanceStatus::ReviewedMapping => DataStatus::ReviewedMapping,
        ProvenanceStatus::ScenarioAssumption => DataStatus::ScenarioAssumption,
        ProvenanceStatus::Derived => DataStatus::Derived,
        ProvenanceStatus::Generated => DataStatus::Generated,
    }
}

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
            status: data_status(record.status),
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
            mass_grams_per_base_unit: record.mass_grams_per_base_unit,
            volume_cm3_per_base_unit: record.volume_cm3_per_base_unit,
            status: data_status(record.status),
        })
        .collect();

    let places = pack
        .pack()
        .places
        .iter()
        .map(|record| PlaceDef {
            id: record.id.clone(),
            name: record.name.clone(),
            kind: record.kind,
            status: data_status(record.status),
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
            status: data_status(record.status),
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
            base_travel_ticks: record.base_travel_ticks,
            status: data_status(record.status),
        })
        .collect();

    ScenarioRegistry::new(units, commodities, places, markets, routes)
}
