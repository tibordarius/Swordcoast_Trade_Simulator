use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{CommodityId, MarketId, PlaceId, RouteEdgeId, UnitId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnitDef {
    pub id: UnitId,
    pub name: String,
    pub dimension: String,
    pub base_unit: UnitId,
    pub numerator: u64,
    pub denominator: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommodityDef {
    pub id: CommodityId,
    pub name: String,
    pub base_unit: UnitId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlaceDef {
    pub id: PlaceId,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarketDef {
    pub id: MarketId,
    pub name: String,
    pub place_id: PlaceId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteDef {
    pub id: RouteEdgeId,
    pub from_market: MarketId,
    pub to_market: MarketId,
    pub bidirectional: bool,
    pub distance_meters: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryError {
    DuplicateUnit(UnitId),
    DuplicateCommodity(CommodityId),
    DuplicatePlace(PlaceId),
    DuplicateMarket(MarketId),
    DuplicateRoute(RouteEdgeId),
    UnknownBaseUnit {
        owner: String,
        unit_id: UnitId,
    },
    UnknownPlace {
        market_id: MarketId,
        place_id: PlaceId,
    },
    UnknownRouteMarket {
        route_id: RouteEdgeId,
        market_id: MarketId,
    },
    InvalidRoute {
        route_id: RouteEdgeId,
    },
    InvalidUnitConversion {
        unit_id: UnitId,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScenarioRegistry {
    units: BTreeMap<UnitId, UnitDef>,
    commodities: BTreeMap<CommodityId, CommodityDef>,
    places: BTreeMap<PlaceId, PlaceDef>,
    markets: BTreeMap<MarketId, MarketDef>,
    routes: BTreeMap<RouteEdgeId, RouteDef>,
}

impl ScenarioRegistry {
    pub fn new(
        units: Vec<UnitDef>,
        commodities: Vec<CommodityDef>,
        places: Vec<PlaceDef>,
        markets: Vec<MarketDef>,
        routes: Vec<RouteDef>,
    ) -> Result<Self, RegistryError> {
        let mut unit_map = BTreeMap::new();
        for unit in units {
            if unit.numerator == 0 || unit.denominator == 0 {
                return Err(RegistryError::InvalidUnitConversion {
                    unit_id: unit.id,
                });
            }
            let id = unit.id.clone();
            if unit_map.insert(id.clone(), unit).is_some() {
                return Err(RegistryError::DuplicateUnit(id));
            }
        }

        for unit in unit_map.values() {
            if !unit_map.contains_key(&unit.base_unit) {
                return Err(RegistryError::UnknownBaseUnit {
                    owner: unit.id.to_string(),
                    unit_id: unit.base_unit.clone(),
                });
            }
        }

        let mut commodity_map = BTreeMap::new();
        for commodity in commodities {
            if !unit_map.contains_key(&commodity.base_unit) {
                return Err(RegistryError::UnknownBaseUnit {
                    owner: commodity.id.to_string(),
                    unit_id: commodity.base_unit,
                });
            }
            let id = commodity.id.clone();
            if commodity_map.insert(id.clone(), commodity).is_some() {
                return Err(RegistryError::DuplicateCommodity(id));
            }
        }

        let mut place_map = BTreeMap::new();
        for place in places {
            let id = place.id.clone();
            if place_map.insert(id.clone(), place).is_some() {
                return Err(RegistryError::DuplicatePlace(id));
            }
        }

        let mut market_map = BTreeMap::new();
        for market in markets {
            if !place_map.contains_key(&market.place_id) {
                return Err(RegistryError::UnknownPlace {
                    market_id: market.id,
                    place_id: market.place_id,
                });
            }
            let id = market.id.clone();
            if market_map.insert(id.clone(), market).is_some() {
                return Err(RegistryError::DuplicateMarket(id));
            }
        }

        let mut route_map = BTreeMap::new();
        for route in routes {
            if route.from_market == route.to_market || route.distance_meters == 0 {
                return Err(RegistryError::InvalidRoute {
                    route_id: route.id,
                });
            }

            for market_id in [&route.from_market, &route.to_market] {
                if !market_map.contains_key(market_id) {
                    return Err(RegistryError::UnknownRouteMarket {
                        route_id: route.id.clone(),
                        market_id: market_id.clone(),
                    });
                }
            }

            let id = route.id.clone();
            if route_map.insert(id.clone(), route).is_some() {
                return Err(RegistryError::DuplicateRoute(id));
            }
        }

        Ok(Self {
            units: unit_map,
            commodities: commodity_map,
            places: place_map,
            markets: market_map,
            routes: route_map,
        })
    }

    #[must_use]
    pub fn units(&self) -> &BTreeMap<UnitId, UnitDef> {
        &self.units
    }

    #[must_use]
    pub fn commodities(&self) -> &BTreeMap<CommodityId, CommodityDef> {
        &self.commodities
    }

    #[must_use]
    pub fn places(&self) -> &BTreeMap<PlaceId, PlaceDef> {
        &self.places
    }

    #[must_use]
    pub fn markets(&self) -> &BTreeMap<MarketId, MarketDef> {
        &self.markets
    }

    #[must_use]
    pub fn routes(&self) -> &BTreeMap<RouteEdgeId, RouteDef> {
        &self.routes
    }
}
