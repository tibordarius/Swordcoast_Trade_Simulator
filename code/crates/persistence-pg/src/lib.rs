use postgres::{Client, NoTls, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sim_core::{decode_world_snapshot, encode_world_snapshot, WorldState, SNAPSHOT_STATE_FORMAT};
use thiserror::Error;

const EVENT_SCHEMA_VERSION: i32 = 1;
const GENESIS_HASH: &str = "GENESIS";

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("postgres error: {0}")]
    Postgres(#[from] postgres::Error),
    #[error("snapshot codec error: {0}")]
    SnapshotCodec(String),
    #[error("integrity error: {0}")]
    Integrity(String),
    #[error("command rejected: {0}")]
    Command(String),
    #[error("persistence record not found: {0}")]
    NotFound(String),
    #[error("numeric conversion failed: {0}")]
    Conversion(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldBranch {
    pub world_id: i64,
    pub branch_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorldCommand {
    AdvanceTicks {
        ticks: u64,
    },
    DispatchTrade {
        route_id: String,
        origin_market: String,
        destination_market: String,
        commodity_id: String,
        quantity_milli: i64,
        capital_mcp: i64,
        min_roi_bps: i64,
    },
}

impl WorldCommand {
    pub fn command_type(&self) -> &'static str {
        match self {
            Self::AdvanceTicks { .. } => "advance_ticks",
            Self::DispatchTrade { .. } => "dispatch_trade",
        }
    }

    pub fn apply(&self, world: &mut WorldState) -> Result<(), PersistenceError> {
        match self {
            Self::AdvanceTicks { ticks } => {
                world.run_ticks(*ticks);
                Ok(())
            }
            Self::DispatchTrade {
                route_id,
                origin_market,
                destination_market,
                commodity_id,
                quantity_milli,
                capital_mcp,
                min_roi_bps,
            } => world
                .dispatch_trade(
                    route_id,
                    origin_market,
                    destination_market,
                    commodity_id,
                    *quantity_milli,
                    *capital_mcp,
                    *min_roi_bps,
                )
                .map(|_| ())
                .map_err(|error| PersistenceError::Command(format!("{error:?}"))),
        }
    }

    fn payload(&self) -> Result<Value, PersistenceError> {
        serde_json::to_value(self)
            .map_err(|error| PersistenceError::Integrity(format!("command JSON: {error}")))
    }

    fn from_parts(command_type: &str, payload: Value) -> Result<Self, PersistenceError> {
        let command: Self = serde_json::from_value(payload).map_err(|error| {
            PersistenceError::Integrity(format!("stored command JSON: {error}"))
        })?;
        if command.command_type() != command_type {
            return Err(PersistenceError::Integrity(format!(
                "command type mismatch: column={command_type}, payload={}",
                command.command_type()
            )));
        }
        Ok(command)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRecord {
    pub sequence: i64,
    pub tick_received: i64,
    pub actor: String,
    pub command_type: String,
    pub payload: Value,
    pub schema_version: i32,
    pub event_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotRecord {
    pub id: i64,
    pub tick: i64,
    pub event_sequence: i64,
    pub simulation_version: String,
    pub state_format: String,
    pub checksum: String,
}

#[derive(Debug)]
struct BranchContext {
    last_event_sequence: i64,
    last_event_hash: String,
    seed: i64,
    current_tick: i64,
    simulation_version: String,
}

pub struct PgPersistence {
    client: Client,
    simulation_version: String,
}

impl PgPersistence {
    pub fn connect(
        connection_string: &str,
        simulation_version: impl Into<String>,
    ) -> Result<Self, PersistenceError> {
        Ok(Self {
            client: Client::connect(connection_string, NoTls)?,
            simulation_version: simulation_version.into(),
        })
    }

    pub fn create_world_branch(
        &mut self,
        world_name: &str,
        seed: u64,
        branch_name: &str,
    ) -> Result<WorldBranch, PersistenceError> {
        let seed_i64 = u64_to_i64(seed, "world seed")?;
        let mut tx = self.client.transaction()?;
        let world_id: i64 = tx
            .query_one(
                "INSERT INTO world (name, seed, simulation_version, current_tick)
                 VALUES ($1, $2, $3, 0)
                 RETURNING id",
                &[&world_name, &seed_i64, &self.simulation_version],
            )?
            .get(0);
        let branch_id: i64 = tx
            .query_one(
                "INSERT INTO branch (world_id, name, canonical)
                 VALUES ($1, $2, TRUE)
                 RETURNING id",
                &[&world_id, &branch_name],
            )?
            .get(0);
        tx.commit()?;
        Ok(WorldBranch {
            world_id,
            branch_id,
        })
    }

    pub fn execute_command(
        &mut self,
        ids: WorldBranch,
        world: &mut WorldState,
        actor: &str,
        command: &WorldCommand,
    ) -> Result<EventRecord, PersistenceError> {
        let mut candidate = world.clone();
        command.apply(&mut candidate)?;

        let payload = command.payload()?;
        let tick_received = u64_to_i64(world.tick(), "event tick")?;
        let candidate_tick = u64_to_i64(candidate.tick(), "candidate tick")?;
        let world_seed = u64_to_i64(world.seed(), "world seed")?;

        let mut tx = self.client.transaction()?;
        let context = lock_branch(&mut tx, ids)?;
        validate_context(
            &context,
            &self.simulation_version,
            world_seed,
            u64_to_i64(world.tick(), "world tick")?,
        )?;

        let sequence = context
            .last_event_sequence
            .checked_add(1)
            .ok_or_else(|| PersistenceError::Conversion("event sequence overflow".to_string()))?;
        let event_hash = compute_event_hash(
            &context.last_event_hash,
            ids.branch_id,
            sequence,
            tick_received,
            actor,
            command.command_type(),
            &payload,
            EVENT_SCHEMA_VERSION,
        )?;

        tx.execute(
            "INSERT INTO input_event_log
               (branch_id, sequence, tick_received, actor, command_type, payload, schema_version, event_hash)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &ids.branch_id,
                &sequence,
                &tick_received,
                &actor,
                &command.command_type(),
                &payload,
                &EVENT_SCHEMA_VERSION,
                &event_hash,
            ],
        )?;
        tx.execute(
            "UPDATE branch
             SET last_event_sequence = $2, last_event_hash = $3
             WHERE id = $1",
            &[&ids.branch_id, &sequence, &event_hash],
        )?;
        tx.execute(
            "UPDATE world SET current_tick = $2 WHERE id = $1",
            &[&ids.world_id, &candidate_tick],
        )?;
        tx.commit()?;

        *world = candidate;
        Ok(EventRecord {
            sequence,
            tick_received,
            actor: actor.to_string(),
            command_type: command.command_type().to_string(),
            payload,
            schema_version: EVENT_SCHEMA_VERSION,
            event_hash,
        })
    }

    pub fn save_snapshot(
        &mut self,
        ids: WorldBranch,
        world: &WorldState,
    ) -> Result<SnapshotRecord, PersistenceError> {
        let bytes = encode_world_snapshot(world)
            .map_err(|error| PersistenceError::SnapshotCodec(error.to_string()))?;
        let checksum = sha256_hex(&bytes);
        let tick = u64_to_i64(world.tick(), "snapshot tick")?;
        let seed = u64_to_i64(world.seed(), "world seed")?;

        let mut tx = self.client.transaction()?;
        let context = lock_branch(&mut tx, ids)?;
        validate_context(&context, &self.simulation_version, seed, tick)?;

        let id: i64 = tx
            .query_one(
                "INSERT INTO snapshot
                   (branch_id, tick, simulation_version, state_format, state_bytes, checksum, event_sequence)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 RETURNING id",
                &[
                    &ids.branch_id,
                    &tick,
                    &self.simulation_version,
                    &SNAPSHOT_STATE_FORMAT,
                    &bytes,
                    &checksum,
                    &context.last_event_sequence,
                ],
            )?
            .get(0);
        tx.commit()?;

        Ok(SnapshotRecord {
            id,
            tick,
            event_sequence: context.last_event_sequence,
            simulation_version: self.simulation_version.clone(),
            state_format: SNAPSHOT_STATE_FORMAT.to_string(),
            checksum,
        })
    }

    pub fn restore_latest(
        &mut self,
        ids: WorldBranch,
        genesis_world: &WorldState,
    ) -> Result<WorldState, PersistenceError> {
        let context = read_branch(&mut self.client, ids)?;
        let genesis_seed = u64_to_i64(genesis_world.seed(), "genesis seed")?;
        if context.simulation_version != self.simulation_version {
            return Err(PersistenceError::Integrity(format!(
                "world simulation version {} does not match adapter {}",
                context.simulation_version, self.simulation_version
            )));
        }
        if context.seed != genesis_seed {
            return Err(PersistenceError::Integrity(format!(
                "genesis seed {} does not match stored seed {}",
                genesis_seed, context.seed
            )));
        }

        let events = load_events(&mut self.client, ids.branch_id)?;
        verify_event_chain(ids.branch_id, &events, &context)?;

        let snapshot_row = self.client.query_opt(
            "SELECT id, tick, event_sequence, simulation_version, state_format, state_bytes, checksum
             FROM snapshot
             WHERE branch_id = $1 AND simulation_version = $2
             ORDER BY tick DESC, id DESC
             LIMIT 1",
            &[&ids.branch_id, &self.simulation_version],
        )?;

        let (mut world, watermark) = if let Some(row) = snapshot_row {
            let snapshot_id: i64 = row.get(0);
            let snapshot_tick: i64 = row.get(1);
            let event_sequence: i64 = row.get(2);
            let simulation_version: String = row.get(3);
            let state_format: String = row.get(4);
            let bytes: Vec<u8> = row.get(5);
            let checksum: String = row.get(6);

            if simulation_version != self.simulation_version {
                return Err(PersistenceError::Integrity(format!(
                    "snapshot {snapshot_id} has incompatible simulation version"
                )));
            }
            if state_format != SNAPSHOT_STATE_FORMAT {
                return Err(PersistenceError::Integrity(format!(
                    "snapshot {snapshot_id} uses unsupported format {state_format}"
                )));
            }
            if sha256_hex(&bytes) != checksum {
                return Err(PersistenceError::Integrity(format!(
                    "snapshot {snapshot_id} checksum mismatch"
                )));
            }
            if event_sequence > context.last_event_sequence {
                return Err(PersistenceError::Integrity(format!(
                    "snapshot {snapshot_id} watermark {event_sequence} is ahead of branch head {}",
                    context.last_event_sequence
                )));
            }

            let restored = decode_world_snapshot(&bytes)
                .map_err(|error| PersistenceError::SnapshotCodec(error.to_string()))?;
            if u64_to_i64(restored.tick(), "restored snapshot tick")? != snapshot_tick {
                return Err(PersistenceError::Integrity(format!(
                    "snapshot {snapshot_id} tick metadata does not match state"
                )));
            }
            if u64_to_i64(restored.seed(), "restored snapshot seed")? != context.seed {
                return Err(PersistenceError::Integrity(format!(
                    "snapshot {snapshot_id} seed does not match world"
                )));
            }
            (restored, event_sequence)
        } else {
            (genesis_world.clone(), 0)
        };

        for event in events.iter().filter(|event| event.sequence > watermark) {
            if u64_to_i64(world.tick(), "replay tick")? != event.tick_received {
                return Err(PersistenceError::Integrity(format!(
                    "event {} expected tick {}, replay is at {}",
                    event.sequence,
                    event.tick_received,
                    world.tick()
                )));
            }
            let command = WorldCommand::from_parts(&event.command_type, event.payload.clone())?;
            command.apply(&mut world)?;
        }

        if u64_to_i64(world.tick(), "restored final tick")? != context.current_tick {
            return Err(PersistenceError::Integrity(format!(
                "restored tick {} does not match stored world tick {}",
                world.tick(),
                context.current_tick
            )));
        }
        Ok(world)
    }

    pub fn branch_head(&mut self, ids: WorldBranch) -> Result<(i64, String), PersistenceError> {
        let context = read_branch(&mut self.client, ids)?;
        Ok((context.last_event_sequence, context.last_event_hash))
    }
}

fn validate_context(
    context: &BranchContext,
    simulation_version: &str,
    seed: i64,
    current_tick: i64,
) -> Result<(), PersistenceError> {
    if context.simulation_version != simulation_version {
        return Err(PersistenceError::Integrity(format!(
            "simulation version mismatch: db={}, runtime={simulation_version}",
            context.simulation_version
        )));
    }
    if context.seed != seed {
        return Err(PersistenceError::Integrity(format!(
            "world seed mismatch: db={}, runtime={seed}",
            context.seed
        )));
    }
    if context.current_tick != current_tick {
        return Err(PersistenceError::Integrity(format!(
            "authoritative tick mismatch: db={}, runtime={current_tick}",
            context.current_tick
        )));
    }
    Ok(())
}

fn lock_branch(
    tx: &mut Transaction<'_>,
    ids: WorldBranch,
) -> Result<BranchContext, PersistenceError> {
    let row = tx
        .query_opt(
            "SELECT b.last_event_sequence, b.last_event_hash, w.seed, w.current_tick, w.simulation_version
             FROM branch b
             JOIN world w ON w.id = b.world_id
             WHERE w.id = $1 AND b.id = $2
             FOR UPDATE OF b, w",
            &[&ids.world_id, &ids.branch_id],
        )?
        .ok_or_else(|| {
            PersistenceError::NotFound(format!(
                "world {} / branch {}",
                ids.world_id, ids.branch_id
            ))
        })?;
    Ok(BranchContext {
        last_event_sequence: row.get(0),
        last_event_hash: row.get(1),
        seed: row.get(2),
        current_tick: row.get(3),
        simulation_version: row.get(4),
    })
}

fn read_branch(client: &mut Client, ids: WorldBranch) -> Result<BranchContext, PersistenceError> {
    let row = client
        .query_opt(
            "SELECT b.last_event_sequence, b.last_event_hash, w.seed, w.current_tick, w.simulation_version
             FROM branch b
             JOIN world w ON w.id = b.world_id
             WHERE w.id = $1 AND b.id = $2",
            &[&ids.world_id, &ids.branch_id],
        )?
        .ok_or_else(|| {
            PersistenceError::NotFound(format!(
                "world {} / branch {}",
                ids.world_id, ids.branch_id
            ))
        })?;
    Ok(BranchContext {
        last_event_sequence: row.get(0),
        last_event_hash: row.get(1),
        seed: row.get(2),
        current_tick: row.get(3),
        simulation_version: row.get(4),
    })
}

fn load_events(client: &mut Client, branch_id: i64) -> Result<Vec<EventRecord>, PersistenceError> {
    client
        .query(
            "SELECT sequence, tick_received, actor, command_type, payload, schema_version, event_hash
             FROM input_event_log
             WHERE branch_id = $1
             ORDER BY sequence",
            &[&branch_id],
        )?
        .into_iter()
        .map(|row| {
            Ok(EventRecord {
                sequence: row.get(0),
                tick_received: row.get(1),
                actor: row.get(2),
                command_type: row.get(3),
                payload: row.get(4),
                schema_version: row.get(5),
                event_hash: row.get(6),
            })
        })
        .collect()
}

fn verify_event_chain(
    branch_id: i64,
    events: &[EventRecord],
    context: &BranchContext,
) -> Result<(), PersistenceError> {
    let mut expected_sequence = 1_i64;
    let mut previous_hash = GENESIS_HASH.to_string();

    for event in events {
        if event.sequence != expected_sequence {
            return Err(PersistenceError::Integrity(format!(
                "missing or reordered event: expected sequence {expected_sequence}, found {}",
                event.sequence
            )));
        }
        let expected_hash = compute_event_hash(
            &previous_hash,
            branch_id,
            event.sequence,
            event.tick_received,
            &event.actor,
            &event.command_type,
            &event.payload,
            event.schema_version,
        )?;
        if expected_hash != event.event_hash {
            return Err(PersistenceError::Integrity(format!(
                "event {} hash mismatch",
                event.sequence
            )));
        }
        previous_hash = event.event_hash.clone();
        expected_sequence = expected_sequence
            .checked_add(1)
            .ok_or_else(|| PersistenceError::Conversion("event sequence overflow".to_string()))?;
    }

    if context.last_event_sequence != events.len() as i64 {
        return Err(PersistenceError::Integrity(format!(
            "branch head sequence {} does not match stored event count {}",
            context.last_event_sequence,
            events.len()
        )));
    }
    let expected_head_hash = events
        .last()
        .map(|event| event.event_hash.as_str())
        .unwrap_or(GENESIS_HASH);
    if context.last_event_hash != expected_head_hash {
        return Err(PersistenceError::Integrity(
            "branch event head hash does not match event log".to_string(),
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn compute_event_hash(
    previous_hash: &str,
    branch_id: i64,
    sequence: i64,
    tick_received: i64,
    actor: &str,
    command_type: &str,
    payload: &Value,
    schema_version: i32,
) -> Result<String, PersistenceError> {
    let payload_bytes = serde_json::to_vec(payload)
        .map_err(|error| PersistenceError::Integrity(format!("event payload JSON: {error}")))?;
    let mut hasher = Sha256::new();
    update_len_prefixed(&mut hasher, previous_hash.as_bytes());
    hasher.update(branch_id.to_le_bytes());
    hasher.update(sequence.to_le_bytes());
    hasher.update(tick_received.to_le_bytes());
    update_len_prefixed(&mut hasher, actor.as_bytes());
    update_len_prefixed(&mut hasher, command_type.as_bytes());
    update_len_prefixed(&mut hasher, &payload_bytes);
    hasher.update(schema_version.to_le_bytes());
    Ok(hex::encode(hasher.finalize()))
}

fn update_len_prefixed(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn u64_to_i64(value: u64, label: &str) -> Result<i64, PersistenceError> {
    i64::try_from(value)
        .map_err(|_| PersistenceError::Conversion(format!("{label} exceeds PostgreSQL BIGINT")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_hash_changes_when_payload_changes() {
        let a = compute_event_hash(
            GENESIS_HASH,
            1,
            1,
            0,
            "dm",
            "advance_ticks",
            &serde_json::json!({"kind":"advance_ticks","ticks":1}),
            1,
        )
        .unwrap();
        let b = compute_event_hash(
            GENESIS_HASH,
            1,
            1,
            0,
            "dm",
            "advance_ticks",
            &serde_json::json!({"kind":"advance_ticks","ticks":2}),
            1,
        )
        .unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn command_column_and_payload_must_agree() {
        let error = WorldCommand::from_parts(
            "dispatch_trade",
            serde_json::json!({"kind":"advance_ticks","ticks":1}),
        )
        .unwrap_err();
        assert!(error.to_string().contains("command type mismatch"));
    }
}
