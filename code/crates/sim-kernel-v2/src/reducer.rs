use crate::{Command, CommandEnvelope, SimTick, WorldRevision, WorldState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplyError {
    TimeRegression {
        current: SimTick,
        requested: SimTick,
    },
    RevisionOverflow,
}

pub struct WorldReducer;

impl WorldReducer {
    pub fn apply(state: &mut WorldState, envelope: &CommandEnvelope) -> Result<(), ApplyError> {
        match envelope.command() {
            Command::AdvanceTo { tick } => Self::advance_to(state, *tick),
        }
    }

    fn advance_to(state: &mut WorldState, requested: SimTick) -> Result<(), ApplyError> {
        let current = state.tick();
        if requested < current {
            return Err(ApplyError::TimeRegression { current, requested });
        }

        let next_revision = state
            .revision()
            .get()
            .checked_add(1)
            .ok_or(ApplyError::RevisionOverflow)?;

        state.commit_tick(requested, WorldRevision::new(next_revision));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ApplyError, WorldReducer};
    use crate::{Command, CommandEnvelope, SimTick, WorldState};

    #[test]
    fn successful_command_advances_time_and_revision_once() {
        let mut state = WorldState::new(7);
        let command =
            CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) });

        WorldReducer::apply(&mut state, &command).unwrap();

        assert_eq!(state.tick(), SimTick::new(12));
        assert_eq!(state.revision().get(), 1);
    }

    #[test]
    fn time_cannot_move_backwards() {
        let mut state = WorldState::new(7);
        WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) }),
        )
        .unwrap();

        let error = WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(2, Command::AdvanceTo { tick: SimTick::new(11) }),
        )
        .unwrap_err();

        assert_eq!(
            error,
            ApplyError::TimeRegression {
                current: SimTick::new(12),
                requested: SimTick::new(11),
            }
        );
        assert_eq!(state.tick(), SimTick::new(12));
        assert_eq!(state.revision().get(), 1);
    }
}
