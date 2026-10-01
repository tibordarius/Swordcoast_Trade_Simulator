//! Toril economy simulation kernel v2.
//!
//! The v2 kernel is built incrementally behind explicit deterministic contracts.

pub mod command;
pub mod hash;
pub mod ids;
pub mod ledger;
pub mod reducer;
pub mod registry;
pub mod replay;
pub mod rng;
pub mod scenario_pack;
pub mod scheduler;
pub mod snapshot;
pub mod state;
pub mod time;
pub mod transaction;
pub mod values;

pub use command::{Command, CommandEnvelope};
pub use hash::fnv1a64;
pub use ids::{
    ActorId, CommodityId, EventId, InventoryAccountId, MarketId, MoneyAccountId, PlaceId,
    RouteEdgeId, ScenarioPackId, ShipmentId, TransactionId, UnitId,
};
pub use ledger::{
    InventoryAccount, InventoryAccountKind, InventoryLedgerEntry, MoneyAccount, MoneyAccountKind,
    MoneyLedgerEntry,
};
pub use reducer::{ApplyError, WorldReducer};
pub use registry::{
    CommodityDef, DataStatus, MarketDef, PlaceDef, PlaceKind, RegistryError, RouteDef,
    ScenarioRegistry, UnitDef,
};
pub use replay::{replay, ReplayError};
pub use rng::{derive_stream_seed, SplitMix64};
pub use scenario_pack::{
    InventoryAccountSeed, LoadedScenario, MoneyAccountSeed, OpeningInventoryBalance,
    ScenarioManifest, ScenarioPack, ScenarioPackError, SCENARIO_PACK_SCHEMA_VERSION,
    SEED_INVENTORY_SOURCE_ACCOUNT, SEED_MONEY_EXTERNAL_ACCOUNT,
};
pub use scheduler::{EventDomain, EventSchedulerState, FiredEvent, ScheduledEvent};
pub use snapshot::{
    decode_snapshot, encode_snapshot, state_hash, SnapshotError, SNAPSHOT_FORMAT_VERSION,
};
pub use state::{WorldRevision, WorldState};
pub use time::SimTick;
pub use transaction::{EconomicTransaction, InventoryPosting, MoneyPosting};
pub use values::{MoneyCp, Quantity, UnitPrice};
