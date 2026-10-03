use std::collections::BTreeMap;

use crate::{
    derive_market_quote, execution_price, ActorId, Command, CommandEnvelope, CommodityId,
    ConsumptionRecord, EconomicTransaction, EventDomain, EventId, EventPayload, InventoryAccount,
    InventoryAccountId, InventoryAccountKind, InventoryPosting, MarketId, MarketListing,
    MarketMathError, MarketObservation, MarketObservationId, MarketSide, MarketTrade, MarketTradeId,
    MoneyAccount, MoneyAccountId,
    MoneyAccountKind, MoneyCp, MoneyPosting, PopulationCohort, PopulationCohortId, ProductionBatch,
    ProductionBatchId, ProductionBatchStatus, ProductionRecipe, ProductionSite, ProductionSiteId,
    Quantity, RecipeId, ScheduledEvent, SimTick, TransactionId, WorldRevision, WorldState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplyError {
    TimeRegression {
        current: SimTick,
        requested: SimTick,
    },
    EventInPast {
        current: SimTick,
        requested: SimTick,
    },
    UnknownActiveEvent(EventId),
    EventSequenceOverflow,
    EventGenerationOverflow(EventId),
    TimeOverflow,
    CycleOverflow,
    RevisionOverflow,
    DuplicateInventoryAccount(InventoryAccountId),
    DuplicateMoneyAccount(MoneyAccountId),
    DuplicateTransaction(TransactionId),
    EmptyTransaction,
    UnknownInventoryAccount(InventoryAccountId),
    UnknownMoneyAccount(MoneyAccountId),
    InventoryNotConserved {
        commodity_id: CommodityId,
        net: Quantity,
    },
    MoneyNotConserved {
        net: MoneyCp,
    },
    NegativeInventoryBalance {
        account_id: InventoryAccountId,
        commodity_id: CommodityId,
        attempted: Quantity,
    },
    NegativeMoneyBalance {
        account_id: MoneyAccountId,
        attempted: MoneyCp,
    },
    ArithmeticOverflow,
    DuplicateRecipe(RecipeId),
    InvalidRecipeQuantity(RecipeId),
    DuplicateProductionSite(ProductionSiteId),
    UnknownRecipe(RecipeId),
    InvalidProductionDuration(ProductionSiteId),
    InvalidInventoryAccountKind {
        account_id: InventoryAccountId,
        expected: InventoryAccountKind,
    },
    ProductionAccountsMustBeDistinct(ProductionSiteId),
    DuplicateProductionBatch(ProductionBatchId),
    UnknownProductionSite(ProductionSiteId),
    UnknownProductionBatch(ProductionBatchId),
    ProductionBatchAlreadyCompleted(ProductionBatchId),
    ProductionBatchNotDue {
        batch_id: ProductionBatchId,
        completes_at: SimTick,
        current: SimTick,
    },
    DuplicatePopulationCohort(PopulationCohortId),
    InvalidPopulationSize(PopulationCohortId),
    InvalidConsumptionQuantity(PopulationCohortId),
    InvalidConsumptionInterval(PopulationCohortId),
    UnknownPopulationCohort(PopulationCohortId),
    DuplicateMarketListing {
        market_id: MarketId,
        commodity_id: CommodityId,
    },
    UnknownMarketListing {
        market_id: MarketId,
        commodity_id: CommodityId,
    },
    InvalidMarketReferencePrice,
    InvalidMarketTargetStock,
    InvalidMarketDepth,
    InvalidMarketSpread,
    InvalidMarketDemandWindow,
    InvalidMoneyAccountKind {
        account_id: MoneyAccountId,
        expected: MoneyAccountKind,
    },
    DuplicateMarketTrade(MarketTradeId),
    DuplicateKnowledgeActor(ActorId),
    UnknownKnowledgeActor(ActorId),
    DuplicateMarketObservation(MarketObservationId),
    InvalidMarketTradeQuantity,
    MarketTradeAccountCollision,
    MarketTradeValueTooSmall,
    MarketMath(MarketMathError),
}

pub struct WorldReducer;

impl WorldReducer {
    pub fn apply(state: &mut WorldState, envelope: &CommandEnvelope) -> Result<(), ApplyError> {
        match envelope.command() {
            Command::AdvanceTo { tick } => Self::advance_to(state, envelope.sequence(), *tick),
            Command::ScheduleEvent {
                event_id,
                at_tick,
                domain,
            } => Self::schedule_event(
                state,
                event_id,
                *at_tick,
                *domain,
                EventPayload::Noop,
            ),
            Command::CancelEvent { event_id } => Self::cancel_event(state, event_id),
            Command::OpenInventoryAccount { account_id, kind } => {
                Self::open_inventory_account(state, account_id, *kind)
            }
            Command::OpenMoneyAccount { account_id, kind } => {
                Self::open_money_account(state, account_id, *kind)
            }
            Command::ApplyTransaction { transaction } => {
                Self::apply_transaction(state, envelope.sequence(), transaction)
            }
            Command::RegisterProductionRecipe { recipe } => {
                Self::register_recipe(state, recipe)
            }
            Command::RegisterProductionSite { site } => Self::register_site(state, site),
            Command::StartProductionBatch { batch_id, site_id } => {
                Self::start_batch(state, envelope.sequence(), batch_id, site_id)
            }
            Command::RegisterPopulationCohort { cohort } => {
                Self::register_cohort(state, cohort)
            }
            Command::RegisterMarketListing { listing } => {
                Self::register_market_listing(state, listing)
            }
            Command::RegisterKnowledgeActor { actor_id } => {
                Self::register_knowledge_actor(state, actor_id)
            }
            Command::DispatchMarketObservation {
                observation_id,
                actor_id,
                market_id,
                commodity_id,
                delay_ticks,
            } => Self::dispatch_market_observation(
                state,
                envelope.sequence(),
                observation_id,
                actor_id,
                market_id,
                commodity_id,
                *delay_ticks,
            ),
            Command::ExecuteMarketTrade {
                trade_id,
                market_id,
                commodity_id,
                side,
                quantity,
                actor_inventory_account,
                actor_money_account,
            } => Self::execute_market_trade(
                state,
                envelope.sequence(),
                trade_id,
                market_id,
                commodity_id,
                *side,
                *quantity,
                actor_inventory_account,
                actor_money_account,
            ),
        }
    }

    fn next_revision(state: &WorldState) -> Result<WorldRevision, ApplyError> {
        state
            .revision()
            .get()
            .checked_add(1)
            .map(WorldRevision::new)
            .ok_or(ApplyError::RevisionOverflow)
    }

    fn advance_to(
        state: &mut WorldState,
        command_sequence: u64,
        requested: SimTick,
    ) -> Result<(), ApplyError> {
        let current = state.tick();
        if requested < current {
            return Err(ApplyError::TimeRegression { current, requested });
        }

        let revision = Self::next_revision(state)?;
        let mut staged = state.clone();

        while let Some(event) = staged.scheduler_mut().pop_next_due(requested) {
            staged.set_tick_unversioned(event.at_tick());
            Self::dispatch_scheduled_event(&mut staged, command_sequence, revision, &event)?;
            staged.scheduler_mut().record_fired(event);
        }

        staged.commit_advance(requested, revision);
        *state = staged;
        Ok(())
    }

    fn dispatch_scheduled_event(
        state: &mut WorldState,
        command_sequence: u64,
        revision: WorldRevision,
        event: &ScheduledEvent,
    ) -> Result<(), ApplyError> {
        match event.payload() {
            EventPayload::Noop => Ok(()),
            EventPayload::ProductionBatchComplete { batch_id } => {
                Self::complete_batch(state, command_sequence, revision, batch_id)
            }
            EventPayload::PopulationConsumptionDue { cohort_id, cycle } => {
                Self::consume_cohort(
                    state,
                    command_sequence,
                    revision,
                    cohort_id,
                    *cycle,
                )
            }
            EventPayload::MarketObservationDelivery { observation } => {
                state.commit_market_observation_delivered(observation.clone(), revision);
                Ok(())
            }
        }
    }

    fn schedule_event(
        state: &mut WorldState,
        event_id: &EventId,
        at_tick: SimTick,
        domain: EventDomain,
        payload: EventPayload,
    ) -> Result<(), ApplyError> {
        let current = state.tick();
        if at_tick < current {
            return Err(ApplyError::EventInPast {
                current,
                requested: at_tick,
            });
        }

        let sequence = state
            .scheduler()
            .next_sequence()
            .ok_or(ApplyError::EventSequenceOverflow)?;
        let generation = state
            .scheduler()
            .next_generation(event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        state.commit_schedule_event(
            ScheduledEvent::new(
                event_id.clone(),
                at_tick,
                domain,
                sequence,
                generation,
                payload,
            ),
            revision,
        );
        Ok(())
    }

    fn schedule_event_with_revision(
        state: &mut WorldState,
        event_id: EventId,
        at_tick: SimTick,
        domain: EventDomain,
        payload: EventPayload,
        revision: WorldRevision,
    ) -> Result<(), ApplyError> {
        let sequence = state
            .scheduler()
            .next_sequence()
            .ok_or(ApplyError::EventSequenceOverflow)?;
        let generation = state
            .scheduler()
            .next_generation(&event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;

        state.commit_schedule_event(
            ScheduledEvent::new(event_id, at_tick, domain, sequence, generation, payload),
            revision,
        );
        Ok(())
    }

    fn cancel_event(state: &mut WorldState, event_id: &EventId) -> Result<(), ApplyError> {
        if !state.scheduler().is_active(event_id) {
            return Err(ApplyError::UnknownActiveEvent(event_id.clone()));
        }

        let generation = state
            .scheduler()
            .next_generation(event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        state.commit_cancel_event(event_id, generation, revision);
        Ok(())
    }

    fn open_inventory_account(
        state: &mut WorldState,
        account_id: &InventoryAccountId,
        kind: InventoryAccountKind,
    ) -> Result<(), ApplyError> {
        if state.inventory_account(account_id).is_some() {
            return Err(ApplyError::DuplicateInventoryAccount(account_id.clone()));
        }

        let revision = Self::next_revision(state)?;
        state.commit_inventory_account(InventoryAccount::new(account_id.clone(), kind), revision);
        Ok(())
    }

    fn open_money_account(
        state: &mut WorldState,
        account_id: &MoneyAccountId,
        kind: MoneyAccountKind,
    ) -> Result<(), ApplyError> {
        if state.money_account(account_id).is_some() {
            return Err(ApplyError::DuplicateMoneyAccount(account_id.clone()));
        }

        let revision = Self::next_revision(state)?;
        state.commit_money_account(MoneyAccount::new(account_id.clone(), kind), revision);
        Ok(())
    }

    fn register_recipe(
        state: &mut WorldState,
        recipe: &ProductionRecipe,
    ) -> Result<(), ApplyError> {
        if state.production_recipe(recipe.id()).is_some() {
            return Err(ApplyError::DuplicateRecipe(recipe.id().clone()));
        }

        if recipe.input_quantity().get() <= 0 || recipe.output_quantity().get() <= 0 {
            return Err(ApplyError::InvalidRecipeQuantity(recipe.id().clone()));
        }

        let revision = Self::next_revision(state)?;
        state.commit_recipe(recipe.clone(), revision);
        Ok(())
    }

    fn register_site(state: &mut WorldState, site: &ProductionSite) -> Result<(), ApplyError> {
        if state.production_site(site.id()).is_some() {
            return Err(ApplyError::DuplicateProductionSite(site.id().clone()));
        }

        if state.production_recipe(site.recipe_id()).is_none() {
            return Err(ApplyError::UnknownRecipe(site.recipe_id().clone()));
        }

        if site.batch_duration_ticks() == 0 {
            return Err(ApplyError::InvalidProductionDuration(site.id().clone()));
        }

        let account_ids = [
            site.input_account(),
            site.wip_account(),
            site.output_account(),
            site.source_sink_account(),
        ];

        let mut distinct = std::collections::BTreeSet::new();
        if !account_ids
            .iter()
            .all(|account_id| distinct.insert((*account_id).clone()))
        {
            return Err(ApplyError::ProductionAccountsMustBeDistinct(site.id().clone()));
        }

        Self::require_inventory_account_kind(
            state,
            site.input_account(),
            InventoryAccountKind::Holding,
        )?;
        Self::require_inventory_account_kind(
            state,
            site.wip_account(),
            InventoryAccountKind::Holding,
        )?;
        Self::require_inventory_account_kind(
            state,
            site.output_account(),
            InventoryAccountKind::Holding,
        )?;
        Self::require_inventory_account_kind(
            state,
            site.source_sink_account(),
            InventoryAccountKind::SourceOrSink,
        )?;

        let revision = Self::next_revision(state)?;
        state.commit_site(site.clone(), revision);
        Ok(())
    }

    fn start_batch(
        state: &mut WorldState,
        command_sequence: u64,
        batch_id: &ProductionBatchId,
        site_id: &ProductionSiteId,
    ) -> Result<(), ApplyError> {
        if state.production_batch(batch_id).is_some() {
            return Err(ApplyError::DuplicateProductionBatch(batch_id.clone()));
        }

        let site = state
            .production_site(site_id)
            .cloned()
            .ok_or_else(|| ApplyError::UnknownProductionSite(site_id.clone()))?;
        let recipe = state
            .production_recipe(site.recipe_id())
            .cloned()
            .ok_or_else(|| ApplyError::UnknownRecipe(site.recipe_id().clone()))?;

        let completes_at = state
            .tick()
            .checked_add(site.batch_duration_ticks())
            .ok_or(ApplyError::TimeOverflow)?;

        let transaction = EconomicTransaction::new(
            TransactionId::new(format!("production.start.{batch_id}")),
            vec![
                InventoryPosting::new(
                    site.input_account().clone(),
                    recipe.input_commodity().clone(),
                    Quantity::new(-recipe.input_quantity().get()),
                ),
                InventoryPosting::new(
                    site.wip_account().clone(),
                    recipe.input_commodity().clone(),
                    recipe.input_quantity(),
                ),
            ],
            vec![],
        );

        Self::validate_transaction(state, &transaction)?;

        let event_id = EventId::new(format!("production.complete.{batch_id}"));
        let event_sequence = state
            .scheduler()
            .next_sequence()
            .ok_or(ApplyError::EventSequenceOverflow)?;
        let event_generation = state
            .scheduler()
            .next_generation(&event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        state.commit_transaction(&transaction, command_sequence, revision);
        state.commit_batch_started(
            ProductionBatch::new(
                batch_id.clone(),
                site.id().clone(),
                recipe.id().clone(),
                state.tick(),
                completes_at,
            ),
            revision,
        );
        state.commit_schedule_event(
            ScheduledEvent::new(
                event_id,
                completes_at,
                EventDomain::Production,
                event_sequence,
                event_generation,
                EventPayload::ProductionBatchComplete {
                    batch_id: batch_id.clone(),
                },
            ),
            revision,
        );

        Ok(())
    }

    fn complete_batch(
        state: &mut WorldState,
        command_sequence: u64,
        revision: WorldRevision,
        batch_id: &ProductionBatchId,
    ) -> Result<(), ApplyError> {
        let batch = state
            .production_batch(batch_id)
            .cloned()
            .ok_or_else(|| ApplyError::UnknownProductionBatch(batch_id.clone()))?;

        if batch.status() == ProductionBatchStatus::Completed {
            return Err(ApplyError::ProductionBatchAlreadyCompleted(batch_id.clone()));
        }

        if state.tick() < batch.completes_at() {
            return Err(ApplyError::ProductionBatchNotDue {
                batch_id: batch_id.clone(),
                completes_at: batch.completes_at(),
                current: state.tick(),
            });
        }

        let site = state
            .production_site(batch.site_id())
            .cloned()
            .ok_or_else(|| ApplyError::UnknownProductionSite(batch.site_id().clone()))?;
        let recipe = state
            .production_recipe(batch.recipe_id())
            .cloned()
            .ok_or_else(|| ApplyError::UnknownRecipe(batch.recipe_id().clone()))?;

        let transaction = EconomicTransaction::new(
            TransactionId::new(format!("production.complete.{batch_id}")),
            vec![
                InventoryPosting::new(
                    site.wip_account().clone(),
                    recipe.input_commodity().clone(),
                    Quantity::new(-recipe.input_quantity().get()),
                ),
                InventoryPosting::new(
                    site.source_sink_account().clone(),
                    recipe.input_commodity().clone(),
                    recipe.input_quantity(),
                ),
                InventoryPosting::new(
                    site.source_sink_account().clone(),
                    recipe.output_commodity().clone(),
                    Quantity::new(-recipe.output_quantity().get()),
                ),
                InventoryPosting::new(
                    site.output_account().clone(),
                    recipe.output_commodity().clone(),
                    recipe.output_quantity(),
                ),
            ],
            vec![],
        );

        Self::validate_transaction(state, &transaction)?;
        state.commit_transaction(&transaction, command_sequence, revision);
        state.commit_batch_completed(batch_id, revision);
        Ok(())
    }

    fn register_cohort(
        state: &mut WorldState,
        cohort: &PopulationCohort,
    ) -> Result<(), ApplyError> {
        if state.population_cohort(cohort.id()).is_some() {
            return Err(ApplyError::DuplicatePopulationCohort(cohort.id().clone()));
        }

        if cohort.population() == 0 {
            return Err(ApplyError::InvalidPopulationSize(cohort.id().clone()));
        }

        if cohort.quantity_per_cycle().get() <= 0 {
            return Err(ApplyError::InvalidConsumptionQuantity(cohort.id().clone()));
        }

        if cohort.interval_ticks() == 0 {
            return Err(ApplyError::InvalidConsumptionInterval(cohort.id().clone()));
        }

        Self::require_inventory_account_kind(
            state,
            cohort.inventory_account(),
            InventoryAccountKind::Holding,
        )?;
        Self::require_inventory_account_kind(
            state,
            cohort.sink_account(),
            InventoryAccountKind::SourceOrSink,
        )?;

        let first_due = state
            .tick()
            .checked_add(cohort.interval_ticks())
            .ok_or(ApplyError::TimeOverflow)?;
        let event_id = EventId::new(format!("population.consume.{}", cohort.id()));
        let event_sequence = state
            .scheduler()
            .next_sequence()
            .ok_or(ApplyError::EventSequenceOverflow)?;
        let event_generation = state
            .scheduler()
            .next_generation(&event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        state.commit_cohort(cohort.clone(), revision);
        state.commit_schedule_event(
            ScheduledEvent::new(
                event_id,
                first_due,
                EventDomain::Population,
                event_sequence,
                event_generation,
                EventPayload::PopulationConsumptionDue {
                    cohort_id: cohort.id().clone(),
                    cycle: 1,
                },
            ),
            revision,
        );

        Ok(())
    }

    fn consume_cohort(
        state: &mut WorldState,
        command_sequence: u64,
        revision: WorldRevision,
        cohort_id: &PopulationCohortId,
        cycle: u64,
    ) -> Result<(), ApplyError> {
        let cohort = state
            .population_cohort(cohort_id)
            .cloned()
            .ok_or_else(|| ApplyError::UnknownPopulationCohort(cohort_id.clone()))?;

        let requested = cohort.quantity_per_cycle();
        let available = state
            .inventory_balance(cohort.inventory_account(), cohort.commodity_id())
            .ok_or_else(|| ApplyError::UnknownInventoryAccount(cohort.inventory_account().clone()))?;

        let served_value = available.get().max(0).min(requested.get());
        let served = Quantity::new(served_value);
        let unmet = Quantity::new(
            requested
                .get()
                .checked_sub(served_value)
                .ok_or(ApplyError::ArithmeticOverflow)?,
        );

        if served_value > 0 {
            let transaction = EconomicTransaction::new(
                TransactionId::new(format!("consumption.{cohort_id}.{cycle}")),
                vec![
                    InventoryPosting::new(
                        cohort.inventory_account().clone(),
                        cohort.commodity_id().clone(),
                        Quantity::new(-served_value),
                    ),
                    InventoryPosting::new(
                        cohort.sink_account().clone(),
                        cohort.commodity_id().clone(),
                        served,
                    ),
                ],
                vec![],
            );

            Self::validate_transaction(state, &transaction)?;
            state.commit_transaction(&transaction, command_sequence, revision);
        }

        state.commit_consumption_record(
            ConsumptionRecord::new(
                cohort_id.clone(),
                cycle,
                state.tick(),
                cohort.commodity_id().clone(),
                requested,
                served,
                unmet,
            ),
            revision,
        );

        let next_cycle = cycle.checked_add(1).ok_or(ApplyError::CycleOverflow)?;
        let next_tick = state
            .tick()
            .checked_add(cohort.interval_ticks())
            .ok_or(ApplyError::TimeOverflow)?;

        Self::schedule_event_with_revision(
            state,
            EventId::new(format!("population.consume.{cohort_id}")),
            next_tick,
            EventDomain::Population,
            EventPayload::PopulationConsumptionDue {
                cohort_id: cohort_id.clone(),
                cycle: next_cycle,
            },
            revision,
        )
    }

    fn register_market_listing(
        state: &mut WorldState,
        listing: &MarketListing,
    ) -> Result<(), ApplyError> {
        if state
            .market_listing(listing.market_id(), listing.commodity_id())
            .is_some()
        {
            return Err(ApplyError::DuplicateMarketListing {
                market_id: listing.market_id().clone(),
                commodity_id: listing.commodity_id().clone(),
            });
        }

        if !listing.reference_price().is_positive() {
            return Err(ApplyError::InvalidMarketReferencePrice);
        }

        if listing.target_stock().get() <= 0 {
            return Err(ApplyError::InvalidMarketTargetStock);
        }

        if listing.depth().get() <= 0 {
            return Err(ApplyError::InvalidMarketDepth);
        }

        if listing.spread_bps() == 0 || listing.spread_bps() > 10_000 {
            return Err(ApplyError::InvalidMarketSpread);
        }

        if listing.demand_window_ticks() == 0 {
            return Err(ApplyError::InvalidMarketDemandWindow);
        }

        Self::require_inventory_account_kind(
            state,
            listing.inventory_account(),
            InventoryAccountKind::Holding,
        )?;
        Self::require_money_account_kind(
            state,
            listing.money_account(),
            MoneyAccountKind::Holding,
        )?;

        let revision = Self::next_revision(state)?;
        state.commit_market_listing(listing.clone(), revision);
        Ok(())
    }

    fn register_knowledge_actor(
        state: &mut WorldState,
        actor_id: &ActorId,
    ) -> Result<(), ApplyError> {
        if state.is_knowledge_actor_registered(actor_id) {
            return Err(ApplyError::DuplicateKnowledgeActor(actor_id.clone()));
        }

        let revision = Self::next_revision(state)?;
        state.commit_knowledge_actor(actor_id.clone(), revision);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn dispatch_market_observation(
        state: &mut WorldState,
        command_sequence: u64,
        observation_id: &MarketObservationId,
        actor_id: &ActorId,
        market_id: &MarketId,
        commodity_id: &CommodityId,
        delay_ticks: u64,
    ) -> Result<(), ApplyError> {
        if !state.is_knowledge_actor_registered(actor_id) {
            return Err(ApplyError::UnknownKnowledgeActor(actor_id.clone()));
        }

        if state.has_market_observation(observation_id) {
            return Err(ApplyError::DuplicateMarketObservation(
                observation_id.clone(),
            ));
        }

        let listing = state
            .market_listing(market_id, commodity_id)
            .ok_or_else(|| ApplyError::UnknownMarketListing {
                market_id: market_id.clone(),
                commodity_id: commodity_id.clone(),
            })?;

        let quote = derive_market_quote(state, listing).map_err(ApplyError::MarketMath)?;
        let delivery_tick = state
            .tick()
            .checked_add(delay_ticks)
            .ok_or(ApplyError::TimeOverflow)?;
        let event_id = EventId::new(format!("information.deliver.{observation_id}"));
        let event_sequence = state
            .scheduler()
            .next_sequence()
            .ok_or(ApplyError::EventSequenceOverflow)?;
        let event_generation = state
            .scheduler()
            .next_generation(&event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        let observation = MarketObservation::new(
            observation_id.clone(),
            actor_id.clone(),
            quote,
            delivery_tick,
            command_sequence,
        );

        state.commit_market_observation_dispatched(observation_id.clone(), revision);
        state.commit_schedule_event(
            ScheduledEvent::new(
                event_id,
                delivery_tick,
                EventDomain::Information,
                event_sequence,
                event_generation,
                EventPayload::MarketObservationDelivery { observation },
            ),
            revision,
        );

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_market_trade(
        state: &mut WorldState,
        command_sequence: u64,
        trade_id: &MarketTradeId,
        market_id: &MarketId,
        commodity_id: &CommodityId,
        side: MarketSide,
        quantity: Quantity,
        actor_inventory_account: &InventoryAccountId,
        actor_money_account: &MoneyAccountId,
    ) -> Result<(), ApplyError> {
        if quantity.get() <= 0 {
            return Err(ApplyError::InvalidMarketTradeQuantity);
        }

        if state.has_market_trade(trade_id) {
            return Err(ApplyError::DuplicateMarketTrade(trade_id.clone()));
        }

        let listing = state
            .market_listing(market_id, commodity_id)
            .cloned()
            .ok_or_else(|| ApplyError::UnknownMarketListing {
                market_id: market_id.clone(),
                commodity_id: commodity_id.clone(),
            })?;

        if actor_inventory_account == listing.inventory_account()
            || actor_money_account == listing.money_account()
        {
            return Err(ApplyError::MarketTradeAccountCollision);
        }

        Self::require_inventory_account_kind(
            state,
            actor_inventory_account,
            InventoryAccountKind::Holding,
        )?;
        Self::require_money_account_kind(
            state,
            actor_money_account,
            MoneyAccountKind::Holding,
        )?;

        let quote = derive_market_quote(state, &listing).map_err(ApplyError::MarketMath)?;
        let average_unit_price =
            execution_price(&quote, side, quantity, listing.depth()).map_err(ApplyError::MarketMath)?;
        let total_value = average_unit_price
            .total_for(quantity)
            .map_err(|_| ApplyError::ArithmeticOverflow)?;

        if total_value.get() <= 0 {
            return Err(ApplyError::MarketTradeValueTooSmall);
        }

        let negative_quantity = quantity
            .get()
            .checked_neg()
            .ok_or(ApplyError::ArithmeticOverflow)?;
        let negative_value = total_value
            .get()
            .checked_neg()
            .ok_or(ApplyError::ArithmeticOverflow)?;

        let (inventory_postings, money_postings) = match side {
            MarketSide::Buy => (
                vec![
                    InventoryPosting::new(
                        listing.inventory_account().clone(),
                        commodity_id.clone(),
                        Quantity::new(negative_quantity),
                    ),
                    InventoryPosting::new(
                        actor_inventory_account.clone(),
                        commodity_id.clone(),
                        quantity,
                    ),
                ],
                vec![
                    MoneyPosting::new(actor_money_account.clone(), MoneyCp::new(negative_value)),
                    MoneyPosting::new(listing.money_account().clone(), total_value),
                ],
            ),
            MarketSide::Sell => (
                vec![
                    InventoryPosting::new(
                        actor_inventory_account.clone(),
                        commodity_id.clone(),
                        Quantity::new(negative_quantity),
                    ),
                    InventoryPosting::new(
                        listing.inventory_account().clone(),
                        commodity_id.clone(),
                        quantity,
                    ),
                ],
                vec![
                    MoneyPosting::new(listing.money_account().clone(), MoneyCp::new(negative_value)),
                    MoneyPosting::new(actor_money_account.clone(), total_value),
                ],
            ),
        };

        let transaction = EconomicTransaction::new(
            TransactionId::new(format!("market.trade.{trade_id}")),
            inventory_postings,
            money_postings,
        );

        Self::validate_transaction(state, &transaction)?;
        let revision = Self::next_revision(state)?;
        state.commit_transaction(&transaction, command_sequence, revision);
        state.commit_market_trade(
            MarketTrade::new(
                trade_id.clone(),
                state.tick(),
                market_id.clone(),
                commodity_id.clone(),
                side,
                quantity,
                average_unit_price,
                total_value,
            ),
            revision,
        );

        Ok(())
    }

    fn require_inventory_account_kind(
        state: &WorldState,
        account_id: &InventoryAccountId,
        expected: InventoryAccountKind,
    ) -> Result<(), ApplyError> {
        let account = state
            .inventory_account(account_id)
            .ok_or_else(|| ApplyError::UnknownInventoryAccount(account_id.clone()))?;

        if account.kind() != expected {
            return Err(ApplyError::InvalidInventoryAccountKind {
                account_id: account_id.clone(),
                expected,
            });
        }

        Ok(())
    }

    fn require_money_account_kind(
        state: &WorldState,
        account_id: &MoneyAccountId,
        expected: MoneyAccountKind,
    ) -> Result<(), ApplyError> {
        let account = state
            .money_account(account_id)
            .ok_or_else(|| ApplyError::UnknownMoneyAccount(account_id.clone()))?;

        if account.kind() != expected {
            return Err(ApplyError::InvalidMoneyAccountKind {
                account_id: account_id.clone(),
                expected,
            });
        }

        Ok(())
    }

    fn apply_transaction(
        state: &mut WorldState,
        command_sequence: u64,
        transaction: &EconomicTransaction,
    ) -> Result<(), ApplyError> {
        Self::validate_transaction(state, transaction)?;
        let revision = Self::next_revision(state)?;
        state.commit_transaction(transaction, command_sequence, revision);
        Ok(())
    }

    fn validate_transaction(
        state: &WorldState,
        transaction: &EconomicTransaction,
    ) -> Result<(), ApplyError> {
        if transaction.is_empty() {
            return Err(ApplyError::EmptyTransaction);
        }

        if state.has_applied_transaction(transaction.id()) {
            return Err(ApplyError::DuplicateTransaction(transaction.id().clone()));
        }

        let mut commodity_net: BTreeMap<CommodityId, i64> = BTreeMap::new();
        let mut inventory_delta: BTreeMap<(InventoryAccountId, CommodityId), i64> = BTreeMap::new();

        for posting in transaction.inventory_postings() {
            if state.inventory_account(&posting.account_id).is_none() {
                return Err(ApplyError::UnknownInventoryAccount(
                    posting.account_id.clone(),
                ));
            }

            let net = commodity_net
                .entry(posting.commodity_id.clone())
                .or_insert(0);
            *net = net
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;

            let delta = inventory_delta
                .entry((posting.account_id.clone(), posting.commodity_id.clone()))
                .or_insert(0);
            *delta = delta
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;
        }

        for (commodity_id, net) in commodity_net {
            if net != 0 {
                return Err(ApplyError::InventoryNotConserved {
                    commodity_id,
                    net: Quantity::new(net),
                });
            }
        }

        for ((account_id, commodity_id), delta) in inventory_delta {
            let account = state
                .inventory_account(&account_id)
                .expect("inventory account was validated above");
            let attempted = account
                .balance(&commodity_id)
                .get()
                .checked_add(delta)
                .ok_or(ApplyError::ArithmeticOverflow)?;

            if account.kind() == InventoryAccountKind::Holding && attempted < 0 {
                return Err(ApplyError::NegativeInventoryBalance {
                    account_id,
                    commodity_id,
                    attempted: Quantity::new(attempted),
                });
            }
        }

        let mut money_net = 0_i128;
        let mut money_delta: BTreeMap<MoneyAccountId, i128> = BTreeMap::new();

        for posting in transaction.money_postings() {
            if state.money_account(&posting.account_id).is_none() {
                return Err(ApplyError::UnknownMoneyAccount(posting.account_id.clone()));
            }

            money_net = money_net
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;

            let delta = money_delta.entry(posting.account_id.clone()).or_insert(0);
            *delta = delta
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;
        }

        if money_net != 0 {
            return Err(ApplyError::MoneyNotConserved {
                net: MoneyCp::new(money_net),
            });
        }

        for (account_id, delta) in money_delta {
            let account = state
                .money_account(&account_id)
                .expect("money account was validated above");
            let attempted = account
                .balance()
                .get()
                .checked_add(delta)
                .ok_or(ApplyError::ArithmeticOverflow)?;

            if account.kind() == MoneyAccountKind::Holding && attempted < 0 {
                return Err(ApplyError::NegativeMoneyBalance {
                    account_id,
                    attempted: MoneyCp::new(attempted),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ApplyError, WorldReducer};
    use crate::{Command, CommandEnvelope, EventDomain, EventId, SimTick, WorldState};

    #[test]
    fn successful_command_advances_time_and_revision_once() {
        let mut state = WorldState::new(7);
        let command =
            CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) });

        WorldReducer::apply(&mut state, &command).unwrap();

        assert_eq!(state.tick(), SimTick::new(12));
        assert_eq!(state.revision().get(), 1);
    }

    #[test]
    fn time_cannot_move_backwards() {
        let mut state = WorldState::new(7);
        WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) }),
        )
        .unwrap();

        let error = WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(2, Command::AdvanceTo { tick: SimTick::new(11) }),
        )
        .unwrap_err();

        assert_eq!(
            error,
            ApplyError::TimeRegression {
                current: SimTick::new(12),
                requested: SimTick::new(11),
            }
        );
        assert_eq!(state.tick(), SimTick::new(12));
        assert_eq!(state.revision().get(), 1);
    }

    #[test]
    fn event_cannot_be_scheduled_in_the_past() {
        let mut state = WorldState::new(7);
        WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) }),
        )
        .unwrap();

        let error = WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(
                2,
                Command::ScheduleEvent {
                    event_id: EventId::new("late"),
                    at_tick: SimTick::new(11),
                    domain: EventDomain::System,
                },
            ),
        )
        .unwrap_err();

        assert_eq!(
            error,
            ApplyError::EventInPast {
                current: SimTick::new(12),
                requested: SimTick::new(11),
            }
        );
    }
}
