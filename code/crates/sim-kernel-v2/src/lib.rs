//! Toril economy simulation kernel v2.
//!
//! Phase 4 begins with deterministic foundational types only.
//! Domain behavior is added incrementally behind explicit contracts.

pub mod command;
pub mod hash;
pub mod ids;
pub mod reducer;
pub mod replay;
pub mod rng;
pub mod snapshot;
pub mod state;
pub mod time;
pub mod values;

pub use command::{Command, CommandEnvelope};
pub use hash::fnv1a64;
pub use ids::{ActorId, CommodityId, MarketId, ShipmentId};
pub use reducer::{ApplyError, WorldReducer};
pub use replay::{replay, ReplayError};
pub use rng::{derive_stream_seed, SplitMix64};
pub use snapshot::{
    decode_snapshot, encode_snapshot, state_hash, SnapshotError, SNAPSHOT_FORMAT_V2,
};
pub use state::{WorldRevision, WorldState};
pub use time::SimTick;
pub use values::{MoneyCp, Quantity, UnitPrice};
