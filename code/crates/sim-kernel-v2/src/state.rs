use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    EconomicTransaction, EventDomain, EventId, EventSchedulerState, InventoryAccount,
    InventoryAccountId, InventoryLedgerEntry, MoneyAccount, MoneyAccountId, MoneyLedgerEntry,
    Quantity, SimTick, TransactionId,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

    pub(crate) fn commit_advance(&mut self, tick: SimTick, revision: WorldRevision) {
        self.scheduler.drain_due(tick);
        self.tick = tick;
        self.revision = revision;
    }

    pub(crate) fn commit_inventory_account(
        &mut self,
        account: InventoryAccount,
        revision: WorldRevision,
    ) {
        self.inventory_accounts.insert(account.id().clone(), account);
        self.revision = revision;
    }

    pub(crate) fn commit_money_account(&mut self, account: MoneyAccount, revision: WorldRevision) {
        self.money_accounts.insert(account.id().clone(), account);
        self.revision = revision;
    }

    pub(crate) fn commit_schedule_event(
        &mut self,
        event_id: EventId,
        at_tick: SimTick,
        domain: EventDomain,
        sequence: u64,
        generation: u64,
        revision: WorldRevision,
    ) {
        self.scheduler
            .commit_schedule(event_id, at_tick, domain, sequence, generation);
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
    }
}
