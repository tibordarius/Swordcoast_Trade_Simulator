use serde::{Deserialize, Serialize};

use crate::{
    ActorId, EconomicTransaction, EventDomain, EventId, InventoryAccountId, InventoryAccountKind,
    MarketId, MarketListing, MarketObservationId, MarketSide, MarketTradeId, MoneyAccountId,
    MoneyAccountKind,
    PopulationCohort, ProductionBatchId, ProductionRecipe, ProductionSite, ProductionSiteId,
    Quantity, SimTick, CommodityId,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    sequence: u64,
    command: Command,
}

impl CommandEnvelope {
    #[must_use]
    pub const fn new(sequence: u64, command: Command) -> Self {
        Self { sequence, command }
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Command {
    AdvanceTo {
        tick: SimTick,
    },
    ScheduleEvent {
        event_id: EventId,
        at_tick: SimTick,
        domain: EventDomain,
    },
    CancelEvent {
        event_id: EventId,
    },
    OpenInventoryAccount {
        account_id: InventoryAccountId,
        kind: InventoryAccountKind,
    },
    OpenMoneyAccount {
        account_id: MoneyAccountId,
        kind: MoneyAccountKind,
    },
    ApplyTransaction {
        transaction: EconomicTransaction,
    },
    RegisterProductionRecipe {
        recipe: ProductionRecipe,
    },
    RegisterProductionSite {
        site: ProductionSite,
    },
    StartProductionBatch {
        batch_id: ProductionBatchId,
        site_id: ProductionSiteId,
    },
    RegisterPopulationCohort {
        cohort: PopulationCohort,
    },
    RegisterMarketListing {
        listing: MarketListing,
    },
    RegisterKnowledgeActor {
        actor_id: ActorId,
    },
    DispatchMarketObservation {
        observation_id: MarketObservationId,
        actor_id: ActorId,
        market_id: MarketId,
        commodity_id: CommodityId,
        delay_ticks: u64,
    },
    ExecuteMarketTrade {
        trade_id: MarketTradeId,
        market_id: MarketId,
        commodity_id: CommodityId,
        side: MarketSide,
        quantity: Quantity,
        actor_inventory_account: InventoryAccountId,
        actor_money_account: MoneyAccountId,
    },
}
