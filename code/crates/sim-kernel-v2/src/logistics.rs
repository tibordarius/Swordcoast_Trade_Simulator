use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    CommodityId, InventoryAccountId, MarketId, Quantity, RouteEdgeId, ShipmentId, SimTick,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommodityTransportProfile {
    commodity_id: CommodityId,
    mass_grams_per_unit: u64,
    volume_cm3_per_unit: u64,
}

impl CommodityTransportProfile {
    #[must_use]
    pub const fn new(
        commodity_id: CommodityId,
        mass_grams_per_unit: u64,
        volume_cm3_per_unit: u64,
    ) -> Self {
        Self {
            commodity_id,
            mass_grams_per_unit,
            volume_cm3_per_unit,
        }
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.commodity_id
    }

    #[must_use]
    pub const fn mass_grams_per_unit(&self) -> u64 {
        self.mass_grams_per_unit
    }

    #[must_use]
    pub const fn volume_cm3_per_unit(&self) -> u64 {
        self.volume_cm3_per_unit
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LogisticsRoute {
    id: RouteEdgeId,
    from_market: MarketId,
    to_market: MarketId,
    bidirectional: bool,
    travel_ticks: u64,
    mass_capacity_grams: u64,
    volume_capacity_cm3: u64,
    open: bool,
}

impl LogisticsRoute {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        id: RouteEdgeId,
        from_market: MarketId,
        to_market: MarketId,
        bidirectional: bool,
        travel_ticks: u64,
        mass_capacity_grams: u64,
        volume_capacity_cm3: u64,
        open: bool,
    ) -> Self {
        Self {
            id,
            from_market,
            to_market,
            bidirectional,
            travel_ticks,
            mass_capacity_grams,
            volume_capacity_cm3,
            open,
        }
    }

    #[must_use]
    pub fn id(&self) -> &RouteEdgeId {
        &self.id
    }

    #[must_use]
    pub fn from_market(&self) -> &MarketId {
        &self.from_market
    }

    #[must_use]
    pub fn to_market(&self) -> &MarketId {
        &self.to_market
    }

    #[must_use]
    pub const fn bidirectional(&self) -> bool {
        self.bidirectional
    }

    #[must_use]
    pub const fn travel_ticks(&self) -> u64 {
        self.travel_ticks
    }

    #[must_use]
    pub const fn mass_capacity_grams(&self) -> u64 {
        self.mass_capacity_grams
    }

    #[must_use]
    pub const fn volume_capacity_cm3(&self) -> u64 {
        self.volume_capacity_cm3
    }

    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    #[must_use]
    pub const fn structurally_fits(&self, mass_grams: u64, volume_cm3: u64) -> bool {
        mass_grams <= self.mass_capacity_grams && volume_cm3 <= self.volume_capacity_cm3
    }

    fn directed_neighbor(&self, from: &MarketId) -> Option<MarketId> {
        if &self.from_market == from {
            Some(self.to_market.clone())
        } else if self.bidirectional && &self.to_market == from {
            Some(self.from_market.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ItineraryLeg {
    route_id: RouteEdgeId,
    from_market: MarketId,
    to_market: MarketId,
    travel_ticks: u64,
}

impl ItineraryLeg {
    #[must_use]
    pub fn route_id(&self) -> &RouteEdgeId {
        &self.route_id
    }

    #[must_use]
    pub fn from_market(&self) -> &MarketId {
        &self.from_market
    }

    #[must_use]
    pub fn to_market(&self) -> &MarketId {
        &self.to_market
    }

    #[must_use]
    pub const fn travel_ticks(&self) -> u64 {
        self.travel_ticks
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Itinerary {
    origin_market: MarketId,
    destination_market: MarketId,
    legs: Vec<ItineraryLeg>,
    total_travel_ticks: u64,
}

impl Itinerary {
    #[must_use]
    pub fn origin_market(&self) -> &MarketId {
        &self.origin_market
    }

    #[must_use]
    pub fn destination_market(&self) -> &MarketId {
        &self.destination_market
    }

    #[must_use]
    pub fn legs(&self) -> &[ItineraryLeg] {
        &self.legs
    }

    #[must_use]
    pub const fn total_travel_ticks(&self) -> u64 {
        self.total_travel_ticks
    }

    #[must_use]
    pub fn leg(&self, index: u32) -> Option<&ItineraryLeg> {
        usize::try_from(index).ok().and_then(|index| self.legs.get(index))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShipmentStatus {
    WaitingForCapacity {
        leg_index: u32,
        market_id: MarketId,
        since_tick: SimTick,
    },
    InTransit {
        leg_index: u32,
        departed_at: SimTick,
        arrives_at: SimTick,
    },
    Delivered {
        delivered_at: SimTick,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Shipment {
    id: ShipmentId,
    commodity_id: CommodityId,
    quantity: Quantity,
    source_inventory_account: InventoryAccountId,
    destination_inventory_account: InventoryAccountId,
    cargo_inventory_account: InventoryAccountId,
    origin_market: MarketId,
    destination_market: MarketId,
    itinerary: Itinerary,
    cargo_mass_grams: u64,
    cargo_volume_cm3: u64,
    dispatch_sequence: u64,
    status: ShipmentStatus,
}

impl Shipment {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        id: ShipmentId,
        commodity_id: CommodityId,
        quantity: Quantity,
        source_inventory_account: InventoryAccountId,
        destination_inventory_account: InventoryAccountId,
        cargo_inventory_account: InventoryAccountId,
        origin_market: MarketId,
        destination_market: MarketId,
        itinerary: Itinerary,
        cargo_mass_grams: u64,
        cargo_volume_cm3: u64,
        dispatch_sequence: u64,
        status: ShipmentStatus,
    ) -> Self {
        Self {
            id,
            commodity_id,
            quantity,
            source_inventory_account,
            destination_inventory_account,
            cargo_inventory_account,
            origin_market,
            destination_market,
            itinerary,
            cargo_mass_grams,
            cargo_volume_cm3,
            dispatch_sequence,
            status,
        }
    }

    #[must_use]
    pub fn id(&self) -> &ShipmentId {
        &self.id
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.commodity_id
    }

    #[must_use]
    pub const fn quantity(&self) -> Quantity {
        self.quantity
    }

    #[must_use]
    pub fn source_inventory_account(&self) -> &InventoryAccountId {
        &self.source_inventory_account
    }

    #[must_use]
    pub fn destination_inventory_account(&self) -> &InventoryAccountId {
        &self.destination_inventory_account
    }

    #[must_use]
    pub fn cargo_inventory_account(&self) -> &InventoryAccountId {
        &self.cargo_inventory_account
    }

    #[must_use]
    pub fn origin_market(&self) -> &MarketId {
        &self.origin_market
    }

    #[must_use]
    pub fn destination_market(&self) -> &MarketId {
        &self.destination_market
    }

    #[must_use]
    pub fn itinerary(&self) -> &Itinerary {
        &self.itinerary
    }

    #[must_use]
    pub const fn cargo_mass_grams(&self) -> u64 {
        self.cargo_mass_grams
    }

    #[must_use]
    pub const fn cargo_volume_cm3(&self) -> u64 {
        self.cargo_volume_cm3
    }

    #[must_use]
    pub const fn dispatch_sequence(&self) -> u64 {
        self.dispatch_sequence
    }

    #[must_use]
    pub fn status(&self) -> &ShipmentStatus {
        &self.status
    }

    pub(crate) fn set_status(&mut self, status: ShipmentStatus) {
        self.status = status;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteCapacityUse {
    reserved_mass_grams: u64,
    reserved_volume_cm3: u64,
    active_shipments: BTreeSet<ShipmentId>,
}

impl RouteCapacityUse {
    #[must_use]
    pub const fn reserved_mass_grams(&self) -> u64 {
        self.reserved_mass_grams
    }

    #[must_use]
    pub const fn reserved_volume_cm3(&self) -> u64 {
        self.reserved_volume_cm3
    }

    #[must_use]
    pub fn active_shipments(&self) -> &BTreeSet<ShipmentId> {
        &self.active_shipments
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LogisticsState {
    profiles: BTreeMap<CommodityId, CommodityTransportProfile>,
    routes: BTreeMap<RouteEdgeId, LogisticsRoute>,
    route_usage: BTreeMap<RouteEdgeId, RouteCapacityUse>,
    shipments: BTreeMap<ShipmentId, Shipment>,
}

#[derive(Clone, Debug)]
struct PathCandidate {
    market: MarketId,
    total_ticks: u64,
    route_ids: Vec<RouteEdgeId>,
    legs: Vec<ItineraryLeg>,
}

impl LogisticsState {
    #[must_use]
    pub fn transport_profile(
        &self,
        commodity_id: &CommodityId,
    ) -> Option<&CommodityTransportProfile> {
        self.profiles.get(commodity_id)
    }

    #[must_use]
    pub fn route(&self, route_id: &RouteEdgeId) -> Option<&LogisticsRoute> {
        self.routes.get(route_id)
    }

    #[must_use]
    pub fn routes(&self) -> &BTreeMap<RouteEdgeId, LogisticsRoute> {
        &self.routes
    }

    #[must_use]
    pub fn route_usage(&self, route_id: &RouteEdgeId) -> RouteCapacityUse {
        self.route_usage.get(route_id).cloned().unwrap_or_default()
    }

    #[must_use]
    pub fn shipment(&self, shipment_id: &ShipmentId) -> Option<&Shipment> {
        self.shipments.get(shipment_id)
    }

    #[must_use]
    pub fn shipments(&self) -> &BTreeMap<ShipmentId, Shipment> {
        &self.shipments
    }

    #[must_use]
    pub fn find_itinerary(
        &self,
        origin: &MarketId,
        destination: &MarketId,
        cargo_mass_grams: u64,
        cargo_volume_cm3: u64,
    ) -> Option<Itinerary> {
        if origin == destination {
            return None;
        }

        let mut frontier = vec![PathCandidate {
            market: origin.clone(),
            total_ticks: 0,
            route_ids: Vec::new(),
            legs: Vec::new(),
        }];
        let mut best: BTreeMap<MarketId, (u64, Vec<RouteEdgeId>)> = BTreeMap::new();
        best.insert(origin.clone(), (0, Vec::new()));

        while !frontier.is_empty() {
            let next_index = frontier
                .iter()
                .enumerate()
                .min_by(|(_, left), (_, right)| compare_candidate(left, right))
                .map(|(index, _)| index)
                .expect("frontier is known non-empty");
            let current = frontier.swap_remove(next_index);

            if &current.market == destination {
                return Some(Itinerary {
                    origin_market: origin.clone(),
                    destination_market: destination.clone(),
                    legs: current.legs,
                    total_travel_ticks: current.total_ticks,
                });
            }

            let mut route_ids: Vec<_> = self.routes.keys().cloned().collect();
            route_ids.sort();

            for route_id in route_ids {
                let route = self
                    .routes
                    .get(&route_id)
                    .expect("route key disappeared during path search");

                if !route.is_open() || !route.structurally_fits(cargo_mass_grams, cargo_volume_cm3) {
                    continue;
                }

                let Some(neighbor) = route.directed_neighbor(&current.market) else {
                    continue;
                };

                let Some(total_ticks) = current.total_ticks.checked_add(route.travel_ticks()) else {
                    continue;
                };

                let mut candidate_route_ids = current.route_ids.clone();
                candidate_route_ids.push(route.id().clone());

                let candidate_key = (total_ticks, candidate_route_ids.clone());
                let improves = best
                    .get(&neighbor)
                    .map(|known| candidate_key < *known)
                    .unwrap_or(true);

                if !improves {
                    continue;
                }

                let mut legs = current.legs.clone();
                legs.push(ItineraryLeg {
                    route_id: route.id().clone(),
                    from_market: current.market.clone(),
                    to_market: neighbor.clone(),
                    travel_ticks: route.travel_ticks(),
                });

                best.insert(neighbor.clone(), candidate_key);
                frontier.push(PathCandidate {
                    market: neighbor,
                    total_ticks,
                    route_ids: candidate_route_ids,
                    legs,
                });
            }
        }

        None
    }

    #[must_use]
    pub fn can_reserve(
        &self,
        route_id: &RouteEdgeId,
        mass_grams: u64,
        volume_cm3: u64,
    ) -> bool {
        let Some(route) = self.route(route_id) else {
            return false;
        };

        if !route.is_open() || !route.structurally_fits(mass_grams, volume_cm3) {
            return false;
        }

        let usage = self.route_usage(route_id);
        usage
            .reserved_mass_grams()
            .checked_add(mass_grams)
            .map(|value| value <= route.mass_capacity_grams())
            .unwrap_or(false)
            && usage
                .reserved_volume_cm3()
                .checked_add(volume_cm3)
                .map(|value| value <= route.volume_capacity_cm3())
                .unwrap_or(false)
    }

    #[must_use]
    pub fn waiting_shipments_for_route(&self, route_id: &RouteEdgeId) -> Vec<ShipmentId> {
        let mut waiting: Vec<_> = self
            .shipments
            .values()
            .filter_map(|shipment| {
                let ShipmentStatus::WaitingForCapacity {
                    leg_index,
                    since_tick,
                    ..
                } = shipment.status()
                else {
                    return None;
                };

                let leg = shipment.itinerary().leg(*leg_index)?;
                if leg.route_id() != route_id {
                    return None;
                }

                Some((
                    *since_tick,
                    shipment.dispatch_sequence(),
                    shipment.id().clone(),
                ))
            })
            .collect();

        waiting.sort();
        waiting.into_iter().map(|(_, _, id)| id).collect()
    }

    pub(crate) fn commit_profile(&mut self, profile: CommodityTransportProfile) {
        self.profiles
            .insert(profile.commodity_id().clone(), profile);
    }

    pub(crate) fn commit_route(&mut self, route: LogisticsRoute) {
        self.routes.insert(route.id().clone(), route);
    }

    pub(crate) fn set_route_open(&mut self, route_id: &RouteEdgeId, open: bool) {
        self.routes
            .get_mut(route_id)
            .expect("validated logistics route missing during state commit")
            .set_open(open);
    }

    pub(crate) fn commit_shipment(&mut self, shipment: Shipment) {
        self.shipments.insert(shipment.id().clone(), shipment);
    }

    pub(crate) fn set_shipment_status(
        &mut self,
        shipment_id: &ShipmentId,
        status: ShipmentStatus,
    ) {
        self.shipments
            .get_mut(shipment_id)
            .expect("validated shipment missing during status update")
            .set_status(status);
    }

    pub(crate) fn reserve_route(
        &mut self,
        route_id: &RouteEdgeId,
        shipment_id: ShipmentId,
        mass_grams: u64,
        volume_cm3: u64,
    ) {
        let usage = self.route_usage.entry(route_id.clone()).or_default();
        usage.reserved_mass_grams = usage
            .reserved_mass_grams
            .checked_add(mass_grams)
            .expect("validated route mass reservation overflowed");
        usage.reserved_volume_cm3 = usage
            .reserved_volume_cm3
            .checked_add(volume_cm3)
            .expect("validated route volume reservation overflowed");
        usage.active_shipments.insert(shipment_id);
    }

    pub(crate) fn release_route(
        &mut self,
        route_id: &RouteEdgeId,
        shipment_id: &ShipmentId,
        mass_grams: u64,
        volume_cm3: u64,
    ) {
        let usage = self
            .route_usage
            .get_mut(route_id)
            .expect("active shipment route usage missing during release");
        usage.reserved_mass_grams = usage
            .reserved_mass_grams
            .checked_sub(mass_grams)
            .expect("route mass reservation underflowed during release");
        usage.reserved_volume_cm3 = usage
            .reserved_volume_cm3
            .checked_sub(volume_cm3)
            .expect("route volume reservation underflowed during release");
        usage.active_shipments.remove(shipment_id);
    }
}

fn compare_candidate(left: &PathCandidate, right: &PathCandidate) -> Ordering {
    left.total_ticks
        .cmp(&right.total_ticks)
        .then_with(|| left.route_ids.cmp(&right.route_ids))
        .then_with(|| left.market.cmp(&right.market))
}

#[cfg(test)]
mod tests {
    use super::{LogisticsRoute, LogisticsState};
    use crate::{MarketId, RouteEdgeId};

    fn route(id: &str, from: &str, to: &str, ticks: u64) -> LogisticsRoute {
        LogisticsRoute::new(
            RouteEdgeId::new(id),
            MarketId::new(from),
            MarketId::new(to),
            true,
            ticks,
            10_000,
            10_000,
            true,
        )
    }

    #[test]
    fn pathfinder_prefers_lowest_total_travel_time() {
        let mut state = LogisticsState::default();
        state.commit_route(route("route.a-b", "a", "b", 5));
        state.commit_route(route("route.b-c", "b", "c", 5));
        state.commit_route(route("route.a-c", "a", "c", 20));

        let itinerary = state
            .find_itinerary(&MarketId::new("a"), &MarketId::new("c"), 1, 1)
            .unwrap();

        assert_eq!(itinerary.total_travel_ticks(), 10);
        assert_eq!(itinerary.legs().len(), 2);
        assert_eq!(
            itinerary.legs()[0].route_id(),
            &RouteEdgeId::new("route.a-b")
        );
    }

    #[test]
    fn equal_cost_paths_use_stable_route_id_tie_break() {
        let mut state = LogisticsState::default();
        state.commit_route(route("route.a-b", "a", "b", 5));
        state.commit_route(route("route.b-d", "b", "d", 5));
        state.commit_route(route("route.a-c", "a", "c", 5));
        state.commit_route(route("route.c-d", "c", "d", 5));

        let itinerary = state
            .find_itinerary(&MarketId::new("a"), &MarketId::new("d"), 1, 1)
            .unwrap();

        let ids: Vec<&str> = itinerary
            .legs()
            .iter()
            .map(|leg| leg.route_id().as_str())
            .collect();
        assert_eq!(ids, vec!["route.a-b", "route.b-d"]);
    }
}
