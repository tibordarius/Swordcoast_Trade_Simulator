use serde::{Deserialize, Serialize};

use crate::{
    CommodityId, InventoryAccountId, PopulationCohortId, Quantity, SimTick,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PopulationCohort {
    id: PopulationCohortId,
    population: u64,
    inventory_account: InventoryAccountId,
    sink_account: InventoryAccountId,
    commodity_id: CommodityId,
    quantity_per_cycle: Quantity,
    interval_ticks: u64,
}

impl PopulationCohort {
    #[must_use]
    pub fn new(
        id: PopulationCohortId,
        population: u64,
        inventory_account: InventoryAccountId,
        sink_account: InventoryAccountId,
        commodity_id: CommodityId,
        quantity_per_cycle: Quantity,
        interval_ticks: u64,
    ) -> Self {
        Self {
            id,
            population,
            inventory_account,
            sink_account,
            commodity_id,
            quantity_per_cycle,
            interval_ticks,
        }
    }

    #[must_use]
    pub fn id(&self) -> &PopulationCohortId {
        &self.id
    }

    #[must_use]
    pub const fn population(&self) -> u64 {
        self.population
    }

    #[must_use]
    pub fn inventory_account(&self) -> &InventoryAccountId {
        &self.inventory_account
    }

    #[must_use]
    pub fn sink_account(&self) -> &InventoryAccountId {
        &self.sink_account
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.commodity_id
    }

    #[must_use]
    pub const fn quantity_per_cycle(&self) -> Quantity {
        self.quantity_per_cycle
    }

    #[must_use]
    pub const fn interval_ticks(&self) -> u64 {
        self.interval_ticks
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionRecord {
    cohort_id: PopulationCohortId,
    cycle: u64,
    tick: SimTick,
    commodity_id: CommodityId,
    requested: Quantity,
    served: Quantity,
    unmet: Quantity,
}

impl ConsumptionRecord {
    #[must_use]
    pub fn new(
        cohort_id: PopulationCohortId,
        cycle: u64,
        tick: SimTick,
        commodity_id: CommodityId,
        requested: Quantity,
        served: Quantity,
        unmet: Quantity,
    ) -> Self {
        Self {
            cohort_id,
            cycle,
            tick,
            commodity_id,
            requested,
            served,
            unmet,
        }
    }

    #[must_use]
    pub fn cohort_id(&self) -> &PopulationCohortId {
        &self.cohort_id
    }

    #[must_use]
    pub const fn cycle(&self) -> u64 {
        self.cycle
    }

    #[must_use]
    pub const fn tick(&self) -> SimTick {
        self.tick
    }

    #[must_use]
    pub fn commodity_id(&self) -> &CommodityId {
        &self.commodity_id
    }

    #[must_use]
    pub const fn requested(&self) -> Quantity {
        self.requested
    }

    #[must_use]
    pub const fn served(&self) -> Quantity {
        self.served
    }

    #[must_use]
    pub const fn unmet(&self) -> Quantity {
        self.unmet
    }
}
