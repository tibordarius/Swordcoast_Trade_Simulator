use serde::{Deserialize, Serialize};

use crate::{
    CommodityId, InventoryAccountId, ProductionBatchId, ProductionSiteId, Quantity, RecipeId,
    SimTick,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionRecipe {
    id: RecipeId,
    input_commodity: CommodityId,
    input_quantity: Quantity,
    output_commodity: CommodityId,
    output_quantity: Quantity,
}

impl ProductionRecipe {
    #[must_use]
    pub fn new(
        id: RecipeId,
        input_commodity: CommodityId,
        input_quantity: Quantity,
        output_commodity: CommodityId,
        output_quantity: Quantity,
    ) -> Self {
        Self {
            id,
            input_commodity,
            input_quantity,
            output_commodity,
            output_quantity,
        }
    }

    #[must_use]
    pub fn id(&self) -> &RecipeId {
        &self.id
    }

    #[must_use]
    pub fn input_commodity(&self) -> &CommodityId {
        &self.input_commodity
    }

    #[must_use]
    pub const fn input_quantity(&self) -> Quantity {
        self.input_quantity
    }

    #[must_use]
    pub fn output_commodity(&self) -> &CommodityId {
        &self.output_commodity
    }

    #[must_use]
    pub const fn output_quantity(&self) -> Quantity {
        self.output_quantity
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionSite {
    id: ProductionSiteId,
    recipe_id: RecipeId,
    input_account: InventoryAccountId,
    wip_account: InventoryAccountId,
    output_account: InventoryAccountId,
    source_sink_account: InventoryAccountId,
    batch_duration_ticks: u64,
}

impl ProductionSite {
    #[must_use]
    pub fn new(
        id: ProductionSiteId,
        recipe_id: RecipeId,
        input_account: InventoryAccountId,
        wip_account: InventoryAccountId,
        output_account: InventoryAccountId,
        source_sink_account: InventoryAccountId,
        batch_duration_ticks: u64,
    ) -> Self {
        Self {
            id,
            recipe_id,
            input_account,
            wip_account,
            output_account,
            source_sink_account,
            batch_duration_ticks,
        }
    }

    #[must_use]
    pub fn id(&self) -> &ProductionSiteId {
        &self.id
    }

    #[must_use]
    pub fn recipe_id(&self) -> &RecipeId {
        &self.recipe_id
    }

    #[must_use]
    pub fn input_account(&self) -> &InventoryAccountId {
        &self.input_account
    }

    #[must_use]
    pub fn wip_account(&self) -> &InventoryAccountId {
        &self.wip_account
    }

    #[must_use]
    pub fn output_account(&self) -> &InventoryAccountId {
        &self.output_account
    }

    #[must_use]
    pub fn source_sink_account(&self) -> &InventoryAccountId {
        &self.source_sink_account
    }

    #[must_use]
    pub const fn batch_duration_ticks(&self) -> u64 {
        self.batch_duration_ticks
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductionBatchStatus {
    InProgress,
    Completed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionBatch {
    id: ProductionBatchId,
    site_id: ProductionSiteId,
    recipe_id: RecipeId,
    started_at: SimTick,
    completes_at: SimTick,
    status: ProductionBatchStatus,
}

impl ProductionBatch {
    #[must_use]
    pub fn new(
        id: ProductionBatchId,
        site_id: ProductionSiteId,
        recipe_id: RecipeId,
        started_at: SimTick,
        completes_at: SimTick,
    ) -> Self {
        Self {
            id,
            site_id,
            recipe_id,
            started_at,
            completes_at,
            status: ProductionBatchStatus::InProgress,
        }
    }

    #[must_use]
    pub fn id(&self) -> &ProductionBatchId {
        &self.id
    }

    #[must_use]
    pub fn site_id(&self) -> &ProductionSiteId {
        &self.site_id
    }

    #[must_use]
    pub fn recipe_id(&self) -> &RecipeId {
        &self.recipe_id
    }

    #[must_use]
    pub const fn started_at(&self) -> SimTick {
        self.started_at
    }

    #[must_use]
    pub const fn completes_at(&self) -> SimTick {
        self.completes_at
    }

    #[must_use]
    pub const fn status(&self) -> ProductionBatchStatus {
        self.status
    }

    pub(crate) fn mark_completed(&mut self) {
        self.status = ProductionBatchStatus::Completed;
    }
}
