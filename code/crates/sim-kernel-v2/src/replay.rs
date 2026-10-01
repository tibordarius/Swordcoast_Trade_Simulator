use crate::{ApplyError, CommandEnvelope, WorldReducer, WorldState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayError {
    NonMonotonicSequence { previous: u64, current: u64 },
    Apply(ApplyError),
}

impl From<ApplyError> for ReplayError {
    fn from(value: ApplyError) -> Self {
        Self::Apply(value)
    }
}

pub fn replay(
    initial: &WorldState,
    commands: &[CommandEnvelope],
) -> Result<WorldState, ReplayError> {
    let mut state = initial.clone();
    let mut previous_sequence = None;

    for envelope in commands {
        if let Some(previous) = previous_sequence {
            if envelope.sequence() <= previous {
                return Err(ReplayError::NonMonotonicSequence {
                    previous,
                    current: envelope.sequence(),
                });
            }
        }

        WorldReducer::apply(&mut state, envelope)?;
        previous_sequence = Some(envelope.sequence());
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::{replay, ReplayError};
    use crate::{state_hash, Command, CommandEnvelope, SimTick, WorldState};

    fn commands() -> Vec<CommandEnvelope> {
        vec![
            CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(5) }),
            CommandEnvelope::new(2, Command::AdvanceTo { tick: SimTick::new(10) }),
        ]
    }

    #[test]
    fn identical_replays_produce_identical_hashes() {
        let initial = WorldState::new(99);
        let a = replay(&initial, &commands()).unwrap();
        let b = replay(&initial, &commands()).unwrap();

        assert_eq!(state_hash(&a).unwrap(), state_hash(&b).unwrap());
        assert_eq!(a, b);
    }

    #[test]
    fn non_monotonic_sequence_is_rejected() {
        let initial = WorldState::new(99);
        let commands = vec![
            CommandEnvelope::new(2, Command::AdvanceTo { tick: SimTick::new(5) }),
            CommandEnvelope::new(2, Command::AdvanceTo { tick: SimTick::new(10) }),
        ];

        assert_eq!(
            replay(&initial, &commands),
            Err(ReplayError::NonMonotonicSequence {
                previous: 2,
                current: 2,
            })
        );
    }
}
