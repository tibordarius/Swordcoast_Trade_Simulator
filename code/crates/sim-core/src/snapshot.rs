use crate::WorldState;

pub const SNAPSHOT_STATE_FORMAT: &str = "sim-core-bincode-v1";

pub fn encode_world_snapshot(world: &WorldState) -> bincode::Result<Vec<u8>> {
    bincode::serialize(world)
}

pub fn decode_world_snapshot(bytes: &[u8]) -> bincode::Result<WorldState> {
    bincode::deserialize(bytes)
}
