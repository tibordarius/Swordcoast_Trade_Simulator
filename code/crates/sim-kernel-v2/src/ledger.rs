use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    CommodityId, InventoryAccountId, MoneyAccountId, MoneyCp, Quantity, SimTick, TransactionId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum InventoryAccountKind {
    Holding,
    SourceOrSink,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoneyAccountKind {
    Holding,
    External,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryAccount {
    id: InventoryAccountId,
    kind: InventoryAccountKind,
    balances: BTreeMap<CommodityId, Quantity>,
}

impl InventoryAccount {
    #[must_use]
    pub fn new(id: InventoryAccountId, kind: InventoryAccountKind) -> Self {
        Self {
            id,
            kind,
            balances: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &InventoryAccountId {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> InventoryAccountKind {
        self.kind
    }

    #[must_use]
    pub fn balance(&self, commodity_id: &CommodityId) -> Quantity {
        self.balances
            .get(commodity_id)
            .copied()
            .unwrap_or(Quantity::ZERO)
    }

    pub(crate) fn apply_delta(&mut self, commodity_id: CommodityId, delta: Quantity) {
        let current = self.balance(&commodity_id).get();
        let next = current
            .checked_add(delta.get())
            .expect("validated inventory posting overflowed during commit");
        self.balances.insert(commodity_id, Quantity::new(next));
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoneyAccount {
    id: MoneyAccountId,
    kind: MoneyAccountKind,
    balance: MoneyCp,
}

impl MoneyAccount {
    #[must_use]
    pub const fn new(id: MoneyAccountId, kind: MoneyAccountKind) -> Self {
        Self {
            id,
            kind,
            balance: MoneyCp::ZERO,
        }
    }

    #[must_use]
    pub fn id(&self) -> &MoneyAccountId {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> MoneyAccountKind {
        self.kind
    }

    #[must_use]
    pub const fn balance(&self) -> MoneyCp {
        self.balance
    }

    pub(crate) fn apply_delta(&mut self, delta: MoneyCp) {
        self.balance = self
            .balance
            .checked_add(delta)
            .expect("validated money posting overflowed during commit");
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryLedgerEntry {
    pub transaction_id: TransactionId,
    pub tick: SimTick,
    pub command_sequence: u64,
    pub account_id: InventoryAccountId,
    pub commodity_id: CommodityId,
    pub delta: Quantity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoneyLedgerEntry {
    pub transaction_id: TransactionId,
    pub tick: SimTick,
    pub command_sequence: u64,
    pub account_id: MoneyAccountId,
    pub delta: MoneyCp,
}
