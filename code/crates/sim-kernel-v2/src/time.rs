use serde::{Deserialize, Serialize};

/// Authoritative simulation time.
///
/// A tick is an integer simulation instant. Calendar interpretation belongs
/// outside this primitive type.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct SimTick(pub u64);

impl SimTick {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    #[must_use]
    pub fn checked_add(self, delta: u64) -> Option<Self> {
        self.0.checked_add(delta).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::SimTick;

    #[test]
    fn tick_addition_is_checked() {
        assert_eq!(SimTick::new(10).checked_add(5), Some(SimTick::new(15)));
        assert_eq!(SimTick::new(u64::MAX).checked_add(1), None);
    }
}
