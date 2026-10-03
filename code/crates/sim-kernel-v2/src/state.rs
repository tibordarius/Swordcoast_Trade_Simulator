use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    ActorId, CommodityId, ConsumptionRecord, EconomicTransaction, EventId, EventSchedulerState,
    InformationState, InventoryAccount, InventoryAccountId, InventoryLedgerEntry, KnowledgeView,
    MarketId, MarketListing, MarketObservation, MarketObservationId, MarketTrade, MarketTradeId,
    MoneyAccount, MoneyAccountId, MoneyLedgerEntry, PopulationCohort, PopulationCohortId,
    ProductionBatch, ProductionBatchId, ProductionRecipe, ProductionSite, ProductionSiteId,
    Quantity, RecipeId, ScheduledEvent, SimTick, TransactionId,
};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct WorldRevision(u64);

impl WorldRevision {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    tick: SimTick,
    world_seed: u64,
    revision: WorldRevision,
    inventory_accounts: BTreeMap<InventoryAccountId, InventoryAccount>,
    money_accounts: BTreeMap<MoneyAccountId, MoneyAccount>,
    inventory_ledger: Vec<InventoryLedgerEntry>,
    money_ledger: Vec<MoneyLedgerEntry>,
    applied_transactions: BTreeSet<TransactionId>,
    scheduler: EventSchedulerState,
    production_recipes: BTreeMap<RecipeId, ProductionRecipe>,
    production_sites: BTreeMap<ProductionSiteId, ProductionSite>,
    production_batches: BTreeMap<ProductionBatchId, ProductionBatch>,
    population_cohorts: BTreeMap<PopulationCohortId, PopulationCohort>,
    consumption_records: Vec<ConsumptionRecord>,
    market_listings: BTreeMap<(MarketId, CommodityId), MarketListing>,
    market_trades: Vec<MarketTrade>,
    market_trade_ids: BTreeSet<MarketTradeId>,
    information: InformationState,
}

impl WorldState {
    #[must_use]
    pub fn new(world_seed: u64) -> Self {
        Self {
            tick: SimTick::ZERO,
            world_seed,
            revision: WorldRevision::ZERO,
            inventory_accounts: BTreeMap::new(),
            money_accounts: BTreeMap::new(),
            inventory_ledger: Vec::new(),
            money_ledger: Vec::new(),
            applied_transactions: BTreeSet::new(),
            scheduler: EventSchedulerState::default(),
            production_recipes: BTreeMap::new(),
            production_sites: BTreeMap::new(),
            production_batches: BTreeMap::new(),
            population_cohorts: BTreeMap::new(),
            consumption_records: Vec::new(),
            market_listings: BTreeMap::new(),
            market_trades: Vec::new(),
            market_trade_ids: BTreeSet::new(),
            information: InformationState::default(),
        }
    }

    #[must_use]
    pub const fn tick(&self) -> SimTick {
        self.tick
    }

    #[must_use]
    pub const fn world_seed(&self) -> u64 {
        self.world_seed
    }

    #[must_use]
    pub const fn revision(&self) -> WorldRevision {
        self.revision
    }

    #[must_use]
    pub fn scheduler(&self) -> &EventSchedulerState {
        &self.scheduler
    }

    #[must_use]
    pub fn inventory_account(&self, account_id: &InventoryAccountId) -> Option<&InventoryAccount> {
        self.inventory_accounts.get(account_id)
    }

    #[must_use]
    pub fn money_account(&self, account_id: &MoneyAccountId) -> Option<&MoneyAccount> {
        self.money_accounts.get(account_id)
    }

    #[must_use]
    pub fn inventory_balance(
        &self,
        account_id: &InventoryAccountId,
        commodity_id: &crate::CommodityId,
    ) -> Option<Quantity> {
        self.inventory_account(account_id)
            .map(|account| account.balance(commodity_id))
    }

    #[must_use]
    pub fn money_balance(&self, account_id: &MoneyAccountId) -> Option<crate::MoneyCp> {
        self.money_account(account_id).map(MoneyAccount::balance)
    }

    #[must_use]
    pub fn inventory_ledger(&self) -> &[InventoryLedgerEntry] {
        &self.inventory_ledger
    }

    #[must_use]
    pub fn money_ledger(&self) -> &[MoneyLedgerEntry] {
        &self.money_ledger
    }

    #[must_use]
    pub fn has_applied_transaction(&self, transaction_id: &TransactionId) -> bool {
        self.applied_transactions.contains(transaction_id)
    }

    #[must_use]
    pub fn production_recipe(&self, recipe_id: &RecipeId) -> Option<&ProductionRecipe> {
        self.production_recipes.get(recipe_id)
    }

    #[must_use]
    pub fn production_site(&self, site_id: &ProductionSiteId) -> Option<&ProductionSite> {
        self.production_sites.get(site_id)
    }

    #[must_use]
    pub fn production_batch(&self, batch_id: &ProductionBatchId) -> Option<&ProductionBatch> {
        self.production_batches.get(batch_id)
    }

    #[must_use]
    pub fn population_cohort(&self, cohort_id: &PopulationCohortId) -> Option<&PopulationCohort> {
        self.population_cohorts.get(cohort_id)
    }

    #[must_use]
    pub fn consumption_records(&self) -> &[ConsumptionRecord] {
        &self.consumption_records
    }

    #[must_use]
    pub fn market_listing(
        &self,
        market_id: &MarketId,
        commodity_id: &CommodityId,
    ) -> Option<&MarketListing> {
        self.market_listings
            .get(&(market_id.clone(), commodity_id.clone()))
    }

    #[must_use]
    pub fn market_trades(&self) -> &[MarketTrade] {
        &self.market_trades
    }

    #[must_use]
    pub fn has_market_trade(&self, trade_id: &MarketTradeId) -> bool {
        self.market_trade_ids.contains(trade_id)
    }

    #[must_use]
    pub fn is_knowledge_actor_registered(&self, actor_id: &ActorId) -> bool {
        self.information.is_actor_registered(actor_id)
    }

    #[must_use]
    pub fn has_market_observation(&self, observation_id: &MarketObservationId) -> bool {
        self.information.has_observation_id(observation_id)
    }

    #[must_use]
    pub fn delivered_market_observations(&self) -> &[MarketObservation] {
        self.information.delivered_observations()
    }

    #[must_use]
    pub fn knowledge_view(&self, actor_id: &ActorId) -> Option<KnowledgeView> {
        self.information.knowledge_view(actor_id, self.tick)
    }

    pub(crate) fn scheduler_mut(&mut self) -> &mut EventSchedulerState {
        &mut self.scheduler
    }

    pub(crate) fn set_tick_unversioned(&mut self, tick: SimTick) {
        self.tick = tick;
    }

    pub(crate) fn commit_advance(&mut self, tick: SimTick, revision: WorldRevision) {
        self.tick = tick;
        self.revision = revision;
    }

    pub(crate) fn commit_inventory_account(
        &mut self,
        account: InventoryAccount,
        revision: WorldRevision,
    ) {
        self.inventory_accounts
            .insert(account.id().clone(), account);
        self.revision = revision;
    }

    pub(crate) fn commit_money_account(&mut self, account: MoneyAccount, revision: WorldRevision) {
        self.money_accounts.insert(account.id().clone(), account);
        self.revision = revision;
    }

    pub(crate) fn commit_schedule_event(&mut self, event: ScheduledEvent, revision: WorldRevision) {
        self.scheduler.commit_scheduled(event);
        self.revision = revision;
    }

    pub(crate) fn commit_cancel_event(
        &mut self,
        event_id: &EventId,
        generation: u64,
        revision: WorldRevision,
    ) {
        self.scheduler.commit_cancel(event_id, generation);
        self.revision = revision;
    }

    pub(crate) fn commit_recipe(&mut self, recipe: ProductionRecipe, revision: WorldRevision) {
        self.production_recipes.insert(recipe.id().clone(), recipe);
        self.revision = revision;
    }

    pub(crate) fn commit_site(&mut self, site: ProductionSite, revision: WorldRevision) {
        self.production_sites.insert(site.id().clone(), site);
        self.revision = revision;
    }

    pub(crate) fn commit_batch_started(&mut self, batch: ProductionBatch, revision: WorldRevision) {
        self.production_batches.insert(batch.id().clone(), batch);
        self.revision = revision;
    }

    pub(crate) fn commit_batch_completed(
        &mut self,
        batch_id: &ProductionBatchId,
        revision: WorldRevision,
    ) {
        self.production_batches
            .get_mut(batch_id)
            .expect("validated production batch missing during completion")
            .mark_completed();
        self.revision = revision;
    }

    pub(crate) fn commit_cohort(&mut self, cohort: PopulationCohort, revision: WorldRevision) {
        self.population_cohorts.insert(cohort.id().clone(), cohort);
        self.revision = revision;
    }

    pub(crate) fn commit_consumption_record(
        &mut self,
        record: ConsumptionRecord,
        revision: WorldRevision,
    ) {
        self.consumption_records.push(record);
        self.revision = revision;
    }

    pub(crate) fn commit_market_listing(
        &mut self,
        listing: MarketListing,
        revision: WorldRevision,
    ) {
        let key = (listing.market_id().clone(), listing.commodity_id().clone());
        self.market_listings.insert(key, listing);
        self.revision = revision;
    }

    pub(crate) fn commit_market_trade(&mut self, trade: MarketTrade, revision: WorldRevision) {
        self.market_trade_ids.insert(trade.trade_id().clone());
        self.market_trades.push(trade);
        self.revision = revision;
    }

    pub(crate) fn commit_knowledge_actor(&mut self, actor_id: ActorId, revision: WorldRevision) {
        self.information.commit_actor(actor_id);
        self.revision = revision;
    }

    pub(crate) fn commit_market_observation_dispatched(
        &mut self,
        observation_id: MarketObservationId,
        revision: WorldRevision,
    ) {
        self.information
            .commit_observation_dispatched(observation_id);
        self.revision = revision;
    }

    pub(crate) fn commit_market_observation_delivered(
        &mut self,
        observation: MarketObservation,
        revision: WorldRevision,
    ) {
        self.information.commit_observation_delivered(observation);
        self.revision = revision;
    }

    pub(crate) fn commit_transaction(
        &mut self,
        transaction: &EconomicTransaction,
        command_sequence: u64,
        revision: WorldRevision,
    ) {
        let tick = self.tick;

        for posting in transaction.inventory_postings() {
            let account = self
                .inventory_accounts
                .get_mut(&posting.account_id)
                .expect("validated inventory account missing during commit");
            account.apply_delta(posting.commodity_id.clone(), posting.delta);
            self.inventory_ledger.push(InventoryLedgerEntry {
                transaction_id: transaction.id().clone(),
                tick,
                command_sequence,
                account_id: posting.account_id.clone(),
                commodity_id: posting.commodity_id.clone(),
                delta: posting.delta,
            });
        }

        for posting in transaction.money_postings() {
            let account = self
                .money_accounts
                .get_mut(&posting.account_id)
                .expect("validated money account missing during commit");
            account.apply_delta(posting.delta);
            self.money_ledger.push(MoneyLedgerEntry {
                transaction_id: transaction.id().clone(),
                tick,
                command_sequence,
                account_id: posting.account_id.clone(),
                delta: posting.delta,
            });
        }

        self.applied_transactions.insert(transaction.id().clone());
        self.revision = revision;
    }
}

#[cfg(test)]
mod tests {
    use super::WorldState;
    use crate::SimTick;

    #[test]
    fn new_world_starts_empty_and_at_zero() {
        let state = WorldState::new(42);
        assert_eq!(state.tick(), SimTick::ZERO);
        assert_eq!(state.revision().get(), 0);
        assert_eq!(state.world_seed(), 42);
        assert!(state.inventory_ledger().is_empty());
        assert!(state.money_ledger().is_empty());
        assert_eq!(state.scheduler().active_event_count(), 0);
        assert!(state.consumption_records().is_empty());
    }
}
