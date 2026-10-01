use serde::{Deserialize, Serialize};

use crate::SimTick;

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
    AdvanceTo { tick: SimTick },
}
