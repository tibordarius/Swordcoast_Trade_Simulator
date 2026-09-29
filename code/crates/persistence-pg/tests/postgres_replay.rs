use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use persistence_pg::{PersistenceError, PgPersistence, WorldCommand};
use postgres::{Client, NoTls};
use sim_core::StableStateHash;
use world_data::SeedBundle;

const VERSION: &str = "test-sim-v1";

fn database_url() -> Option<String> {
    env::var("WDEX_TEST_DATABASE_URL").ok()
}

fn unique_name(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{prefix}-{}-{nanos}", std::process::id())
}

#[test]
fn snapshot_restart_replays_later_events_and_detects_tampering() {
    let Some(url) = database_url() else {
        eprintln!("WDEX_TEST_DATABASE_URL not set; skipping PostgreSQL integration test");
        return;
    };

    let seed = 12_345_u64;
    let genesis = SeedBundle::embedded_mvp().unwrap().instantiate_world(seed);
    let mut world = genesis.clone();
    let mut store = PgPersistence::connect(&url, VERSION).unwrap();
    let ids = store
        .create_world_branch(&unique_name("wdex-replay"), seed, "main")
        .unwrap();

    store
        .execute_command(
            ids,
            &mut world,
            "integration-test",
            &WorldCommand::AdvanceTicks { ticks: 12 },
        )
        .unwrap();
    let snapshot = store.save_snapshot(ids, &world).unwrap();
    assert_eq!(snapshot.event_sequence, 1);
    assert_eq!(snapshot.tick, 12);

    store
        .execute_command(
            ids,
            &mut world,
            "merchant-test",
            &WorldCommand::DispatchTrade {
                route_id: "SEA-ATH-WD".to_string(),
                origin_market: "MKT-ATH".to_string(),
                destination_market: "MKT-WD".to_string(),
                commodity_id: "CMD-GRAIN".to_string(),
                quantity_milli: 100_000_000,
                capital_mcp: 1_000_000_000,
                min_roi_bps: 0,
            },
        )
        .unwrap();
    store
        .execute_command(
            ids,
            &mut world,
            "integration-test",
            &WorldCommand::AdvanceTicks { ticks: 6 },
        )
        .unwrap();

    let expected_hash = world.stable_state_hash();
    let restored = store.restore_latest(ids, &genesis).unwrap();
    assert_eq!(restored, world);
    assert_eq!(restored.stable_state_hash(), expected_hash);

    let (head_sequence, head_hash) = store.branch_head(ids).unwrap();
    assert_eq!(head_sequence, 3);
    assert_ne!(head_hash, "GENESIS");

    let mut raw = Client::connect(&url, NoTls).unwrap();
    raw.batch_execute("ALTER TABLE input_event_log DISABLE TRIGGER USER;")
        .unwrap();
    raw.execute(
        "UPDATE input_event_log SET event_hash = 'tampered' WHERE branch_id = $1 AND sequence = 2",
        &[&ids.branch_id],
    )
    .unwrap();
    raw.batch_execute("ALTER TABLE input_event_log ENABLE TRIGGER USER;")
        .unwrap();

    let error = store.restore_latest(ids, &genesis).unwrap_err();
    assert!(matches!(error, PersistenceError::Integrity(_)));
    assert!(error.to_string().contains("hash mismatch"));

    store.delete_world(ids.world_id).unwrap();
}

#[test]
fn deleting_tail_event_is_detected_by_branch_head() {
    let Some(url) = database_url() else {
        eprintln!("WDEX_TEST_DATABASE_URL not set; skipping PostgreSQL integration test");
        return;
    };

    let seed = 54_321_u64;
    let genesis = SeedBundle::embedded_mvp().unwrap().instantiate_world(seed);
    let mut world = genesis.clone();
    let mut store = PgPersistence::connect(&url, VERSION).unwrap();
    let ids = store
        .create_world_branch(&unique_name("wdex-tail"), seed, "main")
        .unwrap();

    store
        .execute_command(
            ids,
            &mut world,
            "integration-test",
            &WorldCommand::AdvanceTicks { ticks: 1 },
        )
        .unwrap();

    let mut raw = Client::connect(&url, NoTls).unwrap();
    raw.batch_execute("ALTER TABLE input_event_log DISABLE TRIGGER USER;")
        .unwrap();
    raw.execute(
        "DELETE FROM input_event_log WHERE branch_id = $1 AND sequence = 1",
        &[&ids.branch_id],
    )
    .unwrap();
    raw.batch_execute("ALTER TABLE input_event_log ENABLE TRIGGER USER;")
        .unwrap();

    let error = store.restore_latest(ids, &genesis).unwrap_err();
    assert!(matches!(error, PersistenceError::Integrity(_)));
    assert!(error.to_string().contains("branch head sequence"));

    store.delete_world(ids.world_id).unwrap();
}
