use serde::{Deserialize, Serialize};

use crate::{fnv1a64, WorldState};

/// Snapshot schema version for the v2 kernel.
///
/// Version 6 adds knowledge actors, observation IDs, delivered observations, and pending information payloads.
pub const SNAPSHOT_FORMAT_VERSION: u32 = 6;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnapshotError {
    Codec(String),
    UnsupportedFormat(u32),
}

#[derive(Serialize, Deserialize)]
struct SnapshotEnvelope {
    format: u32,
    state: WorldState,
}

pub fn encode_snapshot(state: &WorldState) -> Result<Vec<u8>, SnapshotError> {
    bincode::serialize(&SnapshotEnvelope {
        format: SNAPSHOT_FORMAT_VERSION,
        state: state.clone(),
    })
    .map_err(|error| SnapshotError::Codec(error.to_string()))
}

pub fn decode_snapshot(bytes: &[u8]) -> Result<WorldState, SnapshotError> {
    let envelope: SnapshotEnvelope =
        bincode::deserialize(bytes).map_err(|error| SnapshotError::Codec(error.to_string()))?;

    if envelope.format != SNAPSHOT_FORMAT_VERSION {
        return Err(SnapshotError::UnsupportedFormat(envelope.format));
    }

    Ok(envelope.state)
}

pub fn state_hash(state: &WorldState) -> Result<u64, SnapshotError> {
    let bytes =
        bincode::serialize(state).map_err(|error| SnapshotError::Codec(error.to_string()))?;
    Ok(fnv1a64(&bytes))
}

#[cfg(test)]
mod tests {
    use super::{decode_snapshot, encode_snapshot, state_hash, SnapshotEnvelope, SnapshotError};
    use crate::{Command, CommandEnvelope, SimTick, WorldReducer, WorldState};

    #[test]
    fn snapshot_roundtrip_preserves_hash() {
        let mut state = WorldState::new(1234);
        WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(
                1,
                Command::AdvanceTo {
                    tick: SimTick::new(88),
                },
            ),
        )
        .unwrap();

        let before = state_hash(&state).unwrap();
        let encoded = encode_snapshot(&state).unwrap();
        let decoded = decode_snapshot(&encoded).unwrap();

        assert_eq!(before, state_hash(&decoded).unwrap());
        assert_eq!(state, decoded);
    }

    #[test]
    fn unsupported_snapshot_format_is_rejected() {
        let bytes = bincode::serialize(&SnapshotEnvelope {
            format: 999,
            state: WorldState::new(1),
        })
        .unwrap();

        assert_eq!(
            decode_snapshot(&bytes),
            Err(SnapshotError::UnsupportedFormat(999))
        );
    }
}
