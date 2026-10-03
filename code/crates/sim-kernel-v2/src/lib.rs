//! Toril economy simulation kernel v2.
//!
//! The v2 kernel is built incrementally behind explicit deterministic contracts.

pub mod command;
pub mod hash;
pub mod ids;
pub mod information;
pub mod ledger;
pub mod market;
pub mod population;
pub mod production;
pub mod reducer;
pub mod registry;
pub mod replay;
pub mod rng;
pub mod scheduler;
pub mod snapshot;
pub mod state;
pub mod time;
pub mod transaction;
pub mod values;

pub use command::{Command, CommandEnvelope};
pub use hash::fnv1a64;
pub use ids::{
    ActorId, CommodityId, EventId, InventoryAccountId, MarketId, MarketObservationId,
    MarketTradeId, MoneyAccountId, PlaceId, PopulationCohortId, ProductionBatchId,
    ProductionSiteId, RecipeId, RouteEdgeId, ScenarioPackId, ShipmentId, TransactionId, UnitId,
};
pub use information::{InformationState, KnowledgeView, MarketObservation};
pub use ledger::{
    InventoryAccount, InventoryAccountKind, InventoryLedgerEntry, MoneyAccount, MoneyAccountKind,
    MoneyLedgerEntry,
};
pub use market::{
    derive_market_quote, execution_price, MarketListing, MarketMathError, MarketQuote, MarketSide,
    MarketTrade, PriceExplanation,
};
pub use population::{ConsumptionRecord, PopulationCohort};
pub use production::{ProductionBatch, ProductionBatchStatus, ProductionRecipe, ProductionSite};
pub use reducer::{ApplyError, WorldReducer};
pub use registry::{
    CommodityDef, DataStatus, MarketDef, PlaceDef, PlaceKind, RegistryError, RouteDef,
    ScenarioRegistry, UnitDef,
};
pub use replay::{replay, ReplayError};
pub use rng::{derive_stream_seed, SplitMix64};
pub use scheduler::{EventDomain, EventPayload, EventSchedulerState, FiredEvent, ScheduledEvent};
pub use snapshot::{
    decode_snapshot, encode_snapshot, state_hash, SnapshotError, SNAPSHOT_FORMAT_VERSION,
};
pub use state::{WorldRevision, WorldState};
pub use time::SimTick;
pub use transaction::{EconomicTransaction, InventoryPosting, MoneyPosting};
pub use values::{MoneyCp, Quantity, UnitPrice};
