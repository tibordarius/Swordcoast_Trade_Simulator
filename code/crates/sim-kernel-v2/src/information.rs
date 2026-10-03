use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    ActorId, CommodityId, MarketId, MarketObservationId, MarketQuote, SimTick,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarketObservation {
    observation_id: MarketObservationId,
    actor_id: ActorId,
    quote: MarketQuote,
    delivery_tick: SimTick,
    dispatch_sequence: u64,
}

impl MarketObservation {
    #[must_use]
    pub fn new(
        observation_id: MarketObservationId,
        actor_id: ActorId,
        quote: MarketQuote,
        delivery_tick: SimTick,
        dispatch_sequence: u64,
    ) -> Self {
        Self {
            observation_id,
            actor_id,
            quote,
            delivery_tick,
            dispatch_sequence,
        }
    }

    #[must_use]
    pub fn observation_id(&self) -> &MarketObservationId {
        &self.observation_id
    }

    #[must_use]
    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    #[must_use]
    pub fn quote(&self) -> &MarketQuote {
        &self.quote
    }

    #[must_use]
    pub const fn observed_at(&self) -> SimTick {
        self.quote.tick
    }

    #[must_use]
    pub const fn delivery_tick(&self) -> SimTick {
        self.delivery_tick
    }

    #[must_use]
    pub const fn dispatch_sequence(&self) -> u64 {
        self.dispatch_sequence
    }

    #[must_use]
    pub fn market_id(&self) -> &MarketId {
        &self.quote.market_id
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.quote.commodity_id
    }

    #[must_use]
    pub fn transport_delay_ticks(&self) -> u64 {
        self.delivery_tick
            .get()
            .saturating_sub(self.observed_at().get())
    }

    #[must_use]
    pub fn age_at(&self, as_of: SimTick) -> u64 {
        as_of.get().saturating_sub(self.observed_at().get())
    }

    fn freshness_key(&self) -> (SimTick, u64) {
        (self.observed_at(), self.dispatch_sequence)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeView {
    actor_id: ActorId,
    as_of_tick: SimTick,
    market_observations: BTreeMap<(MarketId, CommodityId), MarketObservation>,
}

impl KnowledgeView {
    #[must_use]
    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    #[must_use]
    pub const fn as_of_tick(&self) -> SimTick {
        self.as_of_tick
    }

    #[must_use]
    pub fn market_observations(
        &self,
    ) -> &BTreeMap<(MarketId, CommodityId), MarketObservation> {
        &self.market_observations
    }

    #[must_use]
    pub fn latest_market_observation(
        &self,
        market_id: &MarketId,
        commodity_id: &CommodityId,
    ) -> Option<&MarketObservation> {
        self.market_observations
            .get(&(market_id.clone(), commodity_id.clone()))
    }

    #[must_use]
    pub fn observation_age_ticks(
        &self,
        market_id: &MarketId,
        commodity_id: &CommodityId,
    ) -> Option<u64> {
        self.latest_market_observation(market_id, commodity_id)
            .map(|observation| observation.age_at(self.as_of_tick))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct InformationState {
    actors: BTreeSet<ActorId>,
    observation_ids: BTreeSet<MarketObservationId>,
    latest_market: BTreeMap<ActorId, BTreeMap<(MarketId, CommodityId), MarketObservation>>,
    delivered_observations: Vec<MarketObservation>,
}

impl InformationState {
    #[must_use]
    pub fn is_actor_registered(&self, actor_id: &ActorId) -> bool {
        self.actors.contains(actor_id)
    }

    #[must_use]
    pub fn has_observation_id(&self, observation_id: &MarketObservationId) -> bool {
        self.observation_ids.contains(observation_id)
    }

    #[must_use]
    pub fn delivered_observations(&self) -> &[MarketObservation] {
        &self.delivered_observations
    }

    #[must_use]
    pub fn knowledge_view(&self, actor_id: &ActorId, as_of_tick: SimTick) -> Option<KnowledgeView> {
        if !self.is_actor_registered(actor_id) {
            return None;
        }

        Some(KnowledgeView {
            actor_id: actor_id.clone(),
            as_of_tick,
            market_observations: self
                .latest_market
                .get(actor_id)
                .cloned()
                .unwrap_or_default(),
        })
    }

    pub(crate) fn commit_actor(&mut self, actor_id: ActorId) {
        self.actors.insert(actor_id);
    }

    pub(crate) fn commit_observation_dispatched(
        &mut self,
        observation_id: MarketObservationId,
    ) {
        self.observation_ids.insert(observation_id);
    }

    pub(crate) fn commit_observation_delivered(&mut self, observation: MarketObservation) {
        let actor_id = observation.actor_id().clone();
        let key = (
            observation.market_id().clone(),
            observation.commodity_id().clone(),
        );

        let actor_market = self.latest_market.entry(actor_id).or_default();
        let should_replace = actor_market
            .get(&key)
            .map(|current| observation.freshness_key() > current.freshness_key())
            .unwrap_or(true);

        if should_replace {
            actor_market.insert(key, observation.clone());
        }

        self.delivered_observations.push(observation);
    }
}

#[cfg(test)]
mod tests {
    use super::{InformationState, MarketObservation};
    use crate::{
        ActorId, CommodityId, MarketId, MarketObservationId, MarketQuote, PriceExplanation,
        Quantity, SimTick, UnitPrice,
    };

    fn observation(id: &str, observed_at: u64, delivered_at: u64, sequence: u64) -> MarketObservation {
        let price = UnitPrice::from_milli_cp(2_000);
        MarketObservation::new(
            MarketObservationId::new(id),
            ActorId::new("actor.1"),
            MarketQuote {
                market_id: MarketId::new("market.waterdeep"),
                commodity_id: CommodityId::new("commodity.grain"),
                tick: SimTick::new(observed_at),
                fundamental: price,
                bid: UnitPrice::from_milli_cp(1_990),
                ask: UnitPrice::from_milli_cp(2_010),
                explanation: PriceExplanation {
                    stock: Quantity::new(100),
                    target_stock: Quantity::new(100),
                    stock_ratio_ppm: 1_000_000,
                    scarcity_factor_ppm: 1_000_000,
                    recent_requested: Quantity::ZERO,
                    recent_unmet: Quantity::ZERO,
                    demand_pressure_ppm: 1_000_000,
                    reference_price: price,
                },
            },
            SimTick::new(delivered_at),
            sequence,
        )
    }

    #[test]
    fn older_late_delivery_does_not_regress_latest_knowledge() {
        let actor = ActorId::new("actor.1");
        let mut state = InformationState::default();
        state.commit_actor(actor.clone());

        state.commit_observation_delivered(observation("newer", 10, 15, 2));
        state.commit_observation_delivered(observation("older", 0, 20, 1));

        let view = state.knowledge_view(&actor, SimTick::new(20)).unwrap();
        let latest = view
            .latest_market_observation(
                &MarketId::new("market.waterdeep"),
                &CommodityId::new("commodity.grain"),
            )
            .unwrap();

        assert_eq!(latest.observation_id(), &MarketObservationId::new("newer"));
        assert_eq!(latest.age_at(SimTick::new(20)), 10);
        assert_eq!(state.delivered_observations().len(), 2);
    }
}
