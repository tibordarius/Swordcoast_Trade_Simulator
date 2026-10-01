//! Toril economy simulation kernel v2.
//!
//! Phase 4 begins with deterministic foundational types only.
//! Domain behavior is added incrementally behind explicit contracts.

pub mod hash;
pub mod ids;
pub mod rng;
pub mod time;
pub mod values;

pub use hash::fnv1a64;
pub use ids::{ActorId, CommodityId, MarketId, ShipmentId};
pub use rng::{derive_stream_seed, SplitMix64};
pub use time::SimTick;
pub use values::{MoneyCp, Quantity, UnitPrice};
