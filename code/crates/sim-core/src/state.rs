use std::collections::BTreeMap;

use crate::{
    derive_stream_seed, evaluate_executable_opportunity, fnv1a64, stable_sort_deltas,
    CalibratedRoute, CargoProfile, CargoUsage, Delta, ExecutableOpportunity, MarketCommodityKey,
    MarketCommodityState, ScheduledEvent, Scheduler, ShipmentStatus, SimulationClock, SplitMix64,
    StableStateHash, TradeShipment,
};

const TICKS_PER_DAY: u64 = 288;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RouteDayKey {
    route_id: String,
    day_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeDispatchError {
    UnknownRoute(String),
    UnsupportedDirection {
        route_id: String,
        origin: String,
        destination: String,
    },
    UnknownCargoProfile(String),
    UnknownMarketState {
        market_id: String,
        commodity_id: String,
    },
    InsufficientOriginStock {
        available_milli: i64,
        requested_milli: i64,
    },
    RouteCapacityExceeded {
        route_id: String,
        day_index: u64,
    },
    OpportunityRejected(ExecutableOpportunity),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldState {
    seed: u64,
    clock: SimulationClock,
    streams: BTreeMap<String, SplitMix64>,
    scheduler: Scheduler,
    deltas: Vec<Delta>,
    markets: BTreeMap<MarketCommodityKey, MarketCommodityState>,
    routes: BTreeMap<String, CalibratedRoute>,
    cargo_profiles: BTreeMap<String, CargoProfile>,
    shipments: BTreeMap<u64, TradeShipment>,
    route_day_usage: BTreeMap<RouteDayKey, CargoUsage>,
    next_sequence: u64,
    next_shipment_id: u64,
    production_signal: i64,
}

impl WorldState {
    const PRODUCTION_EVENT: u32 = 1;
    const PRODUCTION_DELTA: u16 = 1;

    pub fn new(seed: u64) -> Self {
        let mut streams = BTreeMap::new();
        streams.insert(
            "events".to_string(),
            SplitMix64::new(derive_stream_seed(seed, "events")),
        );
        streams.insert(
            "production".to_string(),
            SplitMix64::new(derive_stream_seed(seed, "production")),
        );

        let mut world = Self {
            seed,
            clock: SimulationClock::five_minute(),
            streams,
            scheduler: Scheduler::default(),
            deltas: Vec::new(),
            markets: BTreeMap::new(),
            routes: BTreeMap::new(),
            cargo_profiles: BTreeMap::new(),
            shipments: BTreeMap::new(),
            route_day_usage: BTreeMap::new(),
            next_sequence: 1,
            next_shipment_id: 1,
            production_signal: 0,
        };
        world.schedule(12, Self::PRODUCTION_EVENT);
        world
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn tick(&self) -> u64 {
        self.clock.tick()
    }

    pub fn production_signal(&self) -> i64 {
        self.production_signal
    }

    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }

    pub fn insert_market(
        &mut self,
        key: MarketCommodityKey,
        state: MarketCommodityState,
    ) -> Option<MarketCommodityState> {
        self.markets.insert(key, state)
    }

    pub fn insert_route(&mut self, route: CalibratedRoute) -> Option<CalibratedRoute> {
        self.routes.insert(route.id.clone(), route)
    }

    pub fn insert_cargo_profile(
        &mut self,
        commodity_id: impl Into<String>,
        profile: CargoProfile,
    ) -> Option<CargoProfile> {
        self.cargo_profiles.insert(commodity_id.into(), profile)
    }

    pub fn market(
        &self,
        market_id: &str,
        commodity_id: &str,
    ) -> Option<&MarketCommodityState> {
        self.markets
            .get(&MarketCommodityKey::new(market_id, commodity_id))
    }

    pub fn route(&self, route_id: &str) -> Option<&CalibratedRoute> {
        self.routes.get(route_id)
    }

    pub fn shipment(&self, shipment_id: u64) -> Option<&TradeShipment> {
        self.shipments.get(&shipment_id)
    }

    pub fn shipments(&self) -> impl Iterator<Item = &TradeShipment> {
        self.shipments.values()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn dispatch_trade(
        &mut self,
        route_id: &str,
        origin_market: &str,
        destination_market: &str,
        commodity_id: &str,
        quantity_milli: i64,
        capital_mcp: i64,
        min_roi_bps: i64,
    ) -> Result<TradeShipment, TradeDispatchError> {
        let route = self
            .routes
            .get(route_id)
            .cloned()
            .ok_or_else(|| TradeDispatchError::UnknownRoute(route_id.to_string()))?;

        if !route.supports_direction(origin_market, destination_market) {
            return Err(TradeDispatchError::UnsupportedDirection {
                route_id: route_id.to_string(),
                origin: origin_market.to_string(),
                destination: destination_market.to_string(),
            });
        }

        let cargo = self
            .cargo_profiles
            .get(commodity_id)
            .copied()
            .ok_or_else(|| TradeDispatchError::UnknownCargoProfile(commodity_id.to_string()))?;

        let origin_key = MarketCommodityKey::new(origin_market, commodity_id);
        let destination_key = MarketCommodityKey::new(destination_market, commodity_id);

        let origin = self
            .markets
            .get(&origin_key)
            .ok_or_else(|| TradeDispatchError::UnknownMarketState {
                market_id: origin_market.to_string(),
                commodity_id: commodity_id.to_string(),
            })?;
        let destination = self
            .markets
            .get(&destination_key)
            .ok_or_else(|| TradeDispatchError::UnknownMarketState {
                market_id: destination_market.to_string(),
                commodity_id: commodity_id.to_string(),
            })?;

        if quantity_milli <= 0 || origin.available_milli() < quantity_milli {
            return Err(TradeDispatchError::InsufficientOriginStock {
                available_milli: origin.available_milli(),
                requested_milli: quantity_milli,
            });
        }

        let opportunity = evaluate_executable_opportunity(
            origin.quote(),
            origin.depth_milli(),
            destination.quote(),
            destination.depth_milli(),
            &route,
            cargo,
            quantity_milli,
            capital_mcp,
            min_roi_bps,
        );
        if !opportunity.accepted {
            return Err(TradeDispatchError::OpportunityRejected(opportunity));
        }

        let usage = cargo.usage(quantity_milli);
        let day_index = self.clock.tick() / TICKS_PER_DAY;
        let usage_key = RouteDayKey {
            route_id: route_id.to_string(),
            day_index,
        };
        let combined_usage = self
            .route_day_usage
            .get(&usage_key)
            .copied()
            .unwrap_or_default()
            .checked_add(usage);
        if !route.accepts_usage(combined_usage) {
            return Err(TradeDispatchError::RouteCapacityExceeded {
                route_id: route_id.to_string(),
                day_index,
            });
        }

        let origin = self
            .markets
            .get_mut(&origin_key)
            .expect("origin market validated above");
        if !origin.reserve_for_shipment(quantity_milli)
            || !origin.depart_reserved_shipment(quantity_milli)
        {
            return Err(TradeDispatchError::InsufficientOriginStock {
                available_milli: origin.available_milli(),
                requested_milli: quantity_milli,
            });
        }

        self.markets
            .get_mut(&destination_key)
            .expect("destination market validated above")
            .commit_incoming(quantity_milli);

        let shipment_id = self.next_shipment_id;
        self.next_shipment_id = self
            .next_shipment_id
            .checked_add(1)
            .expect("shipment id overflow");
        let departure_tick = self.clock.tick();
        let eta_tick = departure_tick
            .checked_add(route.travel_ticks)
            .expect("shipment ETA overflow");

        let shipment = TradeShipment::new(
            shipment_id,
            route_id,
            origin_market,
            destination_market,
            commodity_id,
            quantity_milli,
            departure_tick,
            eta_tick,
            opportunity.purchase_mcp,
            opportunity.freight_mcp,
            opportunity.expected_revenue_mcp,
            opportunity.expected_loss_mcp,
        );

        self.route_day_usage.insert(usage_key, combined_usage);
        self.shipments.insert(shipment_id, shipment.clone());
        Ok(shipment)
    }

    pub fn draw_from_stream(&mut self, namespace: &str) -> u64 {
        self.streams
            .get_mut(namespace)
            .unwrap_or_else(|| panic!("unknown RNG stream: {namespace}"))
            .next_u64()
    }

    fn allocate_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .expect("sequence overflow");
        sequence
    }

    fn schedule(&mut self, due_tick: u64, kind: u32) {
        let event = ScheduledEvent {
            due_tick,
            sequence: self.allocate_sequence(),
            kind,
        };
        self.scheduler.push(event);
    }

    fn emit_delta(&mut self, priority: u16, entity_id: u64, kind: u16, amount: i64) {
        let delta = Delta {
            priority,
            entity_id,
            kind,
            sequence: self.allocate_sequence(),
            amount,
        };
        self.deltas.push(delta);
    }

    fn handle_event(&mut self, event: ScheduledEvent) {
        match event.kind {
            Self::PRODUCTION_EVENT => {
                let draw = self.draw_from_stream("production");
                let amount = (draw % 7) as i64 - 3;
                self.emit_delta(100, 1, Self::PRODUCTION_DELTA, amount);
                self.schedule(event.due_tick + 12, Self::PRODUCTION_EVENT);
            }
            other => panic!("unknown event kind: {other}"),
        }
    }

    fn settle_arrivals(&mut self) {
        let current_tick = self.clock.tick();
        let due: Vec<_> = self
            .shipments
            .iter()
            .filter_map(|(id, shipment)| {
                (shipment.status == ShipmentStatus::InTransit
                    && shipment.eta_tick <= current_tick)
                    .then_some((
                        *id,
                        shipment.destination_market.clone(),
                        shipment.commodity_id.clone(),
                        shipment.quantity_milli,
                    ))
            })
            .collect();

        for (shipment_id, destination_market, commodity_id, quantity_milli) in due {
            let destination_key =
                MarketCommodityKey::new(&destination_market, &commodity_id);
            let destination = self
                .markets
                .get_mut(&destination_key)
                .expect("shipment destination market must exist");
            assert!(
                destination.receive_incoming(quantity_milli),
                "shipment arrival must match committed incoming stock"
            );
            self.shipments
                .get_mut(&shipment_id)
                .expect("due shipment must exist")
                .status = ShipmentStatus::Arrived;
        }
    }

    fn advance_markets_if_due(&mut self) {
        if !self.clock.is_hour_boundary() {
            return;
        }
        let elapsed_minutes = self
            .clock
            .tick()
            .checked_mul(u64::from(self.clock.tick_minutes()))
            .expect("elapsed simulation minutes overflow");
        let hour_index = elapsed_minutes / 60;
        for market in self.markets.values_mut() {
            market.advance_hour(hour_index);
        }
    }

    fn commit_deltas(&mut self) {
        stable_sort_deltas(&mut self.deltas);
        for delta in self.deltas.drain(..) {
            match (delta.entity_id, delta.kind) {
                (1, Self::PRODUCTION_DELTA) => self.production_signal += delta.amount,
                _ => panic!("unknown delta: {delta:?}"),
            }
        }
    }

    pub fn advance_one(&mut self) {
        self.clock.advance_one();
        while let Some(event) = self.scheduler.pop_due(self.clock.tick()) {
            self.handle_event(event);
        }
        self.settle_arrivals();
        self.advance_markets_if_due();
        self.commit_deltas();
    }

    pub fn run_ticks(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.advance_one();
        }
    }
}

impl StableStateHash for WorldState {
    fn stable_state_hash(&self) -> u64 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.tick().to_le_bytes());
        bytes.extend_from_slice(&self.seed.to_le_bytes());
        bytes.extend_from_slice(&self.production_signal.to_le_bytes());
        bytes.extend_from_slice(&self.next_sequence.to_le_bytes());
        bytes.extend_from_slice(&self.next_shipment_id.to_le_bytes());

        for (namespace, stream) in &self.streams {
            append_string(&mut bytes, namespace);
            bytes.extend_from_slice(&stream.state().to_le_bytes());
        }

        for event in self.scheduler.sorted_events() {
            bytes.extend_from_slice(&event.due_tick.to_le_bytes());
            bytes.extend_from_slice(&event.sequence.to_le_bytes());
            bytes.extend_from_slice(&event.kind.to_le_bytes());
        }

        for (key, market) in &self.markets {
            key.append_stable_bytes(&mut bytes);
            market.append_stable_bytes(&mut bytes);
        }

        for (commodity_id, profile) in &self.cargo_profiles {
            append_string(&mut bytes, commodity_id);
            bytes.extend_from_slice(&profile.mass_grams_per_base_unit.to_le_bytes());
            bytes.extend_from_slice(&profile.volume_cm3_per_base_unit.to_le_bytes());
        }

        for route in self.routes.values() {
            append_string(&mut bytes, &route.id);
            append_string(&mut bytes, &route.from_market);
            append_string(&mut bytes, &route.to_market);
            bytes.push(u8::from(route.bidirectional));
            bytes.extend_from_slice(&route.travel_ticks.to_le_bytes());
            bytes.extend_from_slice(&route.risk_bps.to_le_bytes());
            bytes.extend_from_slice(&route.freight_mcp_per_kg.to_le_bytes());
            bytes.extend_from_slice(&route.capacity_kg_per_day.to_le_bytes());
            bytes.extend_from_slice(&route.capacity_m3_per_day.to_le_bytes());
        }

        for (key, usage) in &self.route_day_usage {
            append_string(&mut bytes, &key.route_id);
            bytes.extend_from_slice(&key.day_index.to_le_bytes());
            bytes.extend_from_slice(&usage.mass_grams.to_le_bytes());
            bytes.extend_from_slice(&usage.volume_cm3.to_le_bytes());
        }

        for shipment in self.shipments.values() {
            shipment.append_stable_bytes(&mut bytes);
        }

        fnv1a64(&bytes)
    }
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    let encoded = value.as_bytes();
    let len = u16::try_from(encoded.len()).expect("state string too long");
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(encoded);
}
