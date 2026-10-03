use serde::{Deserialize, Serialize};

use crate::{CommodityId, InventoryAccountId, MoneyAccountId, MoneyCp, Quantity, TransactionId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryPosting {
    pub account_id: InventoryAccountId,
    pub commodity_id: CommodityId,
    pub delta: Quantity,
}

impl InventoryPosting {
    #[must_use]
    pub fn new(account_id: InventoryAccountId, commodity_id: CommodityId, delta: Quantity) -> Self {
        Self {
            account_id,
            commodity_id,
            delta,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoneyPosting {
    pub account_id: MoneyAccountId,
    pub delta: MoneyCp,
}

impl MoneyPosting {
    #[must_use]
    pub const fn new(account_id: MoneyAccountId, delta: MoneyCp) -> Self {
        Self { account_id, delta }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EconomicTransaction {
    id: TransactionId,
    inventory_postings: Vec<InventoryPosting>,
    money_postings: Vec<MoneyPosting>,
}

impl EconomicTransaction {
    #[must_use]
    pub fn new(
        id: TransactionId,
        inventory_postings: Vec<InventoryPosting>,
        money_postings: Vec<MoneyPosting>,
    ) -> Self {
        Self {
            id,
            inventory_postings,
            money_postings,
        }
    }

    #[must_use]
    pub fn id(&self) -> &TransactionId {
        &self.id
    }

    #[must_use]
    pub fn inventory_postings(&self) -> &[InventoryPosting] {
        &self.inventory_postings
    }

    #[must_use]
    pub fn money_postings(&self) -> &[MoneyPosting] {
        &self.money_postings
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inventory_postings.is_empty() && self.money_postings.is_empty()
    }
}
