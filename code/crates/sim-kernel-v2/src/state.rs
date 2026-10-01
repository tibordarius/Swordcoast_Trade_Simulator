use serde::{Deserialize, Serialize};

use crate::SimTick;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WorldRevision(u64);

impl WorldRevision {
    pub const ZERO: Self = Self(0);
    #[must_use] pub const fn new(value: u64) -> Self { Self(value) }
    #[must_use] pub const fn get(self) -> u64 { self.0 }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    tick: SimTick,
    world_seed: u64,
    revision: WorldRevision,
}

impl WorldState {
    #[must_use]
    pub const fn new(world_seed: u64) -> Self {
        Self { tick: SimTick::ZERO, world_seed, revision: WorldRevision::ZERO }
    }

    #[must_use] pub const fn tick(&self) -> SimTick { self.tick }
    #[must_use] pub const fn world_seed(&self) -> u64 { self.world_seed }
    #[must_use] pub const fn revision(&self) -> WorldRevision { self.revision }

    pub(crate) fn commit_tick(&mut self, tick: SimTick, revision: WorldRevision) {
        self.tick = tick;
        self.revision = revision;
    }
}

#[cfg(test)]
mod tests {
    use super::WorldState;
    use crate::SimTick;

    #[test]
    fn new_world_starts_at_zero_without_public_mutation() {
        let state = WorldState::new(42);
        assert_eq!(state.tick(), SimTick::ZERO);
        assert_eq!(state.revision().get(), 0);
        assert_eq!(state.world_seed(), 42);
    }
}