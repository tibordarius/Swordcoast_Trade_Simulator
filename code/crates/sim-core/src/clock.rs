use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationClock {
    tick: u64,
    tick_minutes: u16,
}

impl SimulationClock {
    pub const DEFAULT_TICK_MINUTES: u16 = 5;

    pub fn new(tick_minutes: u16) -> Self {
        assert!(tick_minutes > 0, "tick_minutes must be positive");
        Self {
            tick: 0,
            tick_minutes,
        }
    }

    pub fn five_minute() -> Self {
        Self::new(Self::DEFAULT_TICK_MINUTES)
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }
    pub fn tick_minutes(&self) -> u16 {
        self.tick_minutes
    }

    pub fn advance_one(&mut self) {
        self.tick = self.tick.checked_add(1).expect("simulation tick overflow");
    }

    pub fn is_hour_boundary(&self) -> bool {
        let ticks_per_hour = 60_u64 / u64::from(self.tick_minutes);
        ticks_per_hour > 0 && self.tick.is_multiple_of(ticks_per_hour)
    }
}

impl Default for SimulationClock {
    fn default() -> Self {
        Self::five_minute()
    }
}
