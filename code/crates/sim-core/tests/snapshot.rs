use sim_core::{
    decode_world_snapshot, encode_world_snapshot, CalibratedRoute, CargoProfile,
    MarketCommodityKey, MarketCommodityState, StableStateHash, WorldState,
    SNAPSHOT_STATE_FORMAT,
};

fn world_with_active_voyage() -> WorldState {
    let mut world = WorldState::new(12_345);
    world.insert_cargo_profile("CMD-GRAIN", CargoProfile::new(1_000, 1_300));
    world.insert_route(CalibratedRoute::new(
        "SEA-ATH-WD",
        "MKT-ATH",
        "MKT-WD",
        true,
        18,
        100,
        300,
        500_000,
        1_000_000,
    ));
    world.insert_market(
        MarketCommodityKey::new("MKT-ATH", "CMD-GRAIN"),
        MarketCommodityState::new(
            950_000_000,
            700_000_000,
            2_000,
            0,
            25_000_000,
            4,
            300_000_000,
            0,
            0,
        ),
    );
    world.insert_market(
        MarketCommodityKey::new("MKT-WD", "CMD-GRAIN"),
        MarketCommodityState::new(
            640_000_000,
            1_200_000_000,
            2_000,
            0,
            45_000_000,
            5,
            250_000_000,
            0,
            0,
        ),
    );
    world
        .dispatch_trade(
            "SEA-ATH-WD",
            "MKT-ATH",
            "MKT-WD",
            "CMD-GRAIN",
            100_000_000,
            500_000_000,
            500,
        )
        .unwrap();
    world.run_ticks(7);
    world
}

#[test]
fn snapshot_roundtrip_preserves_complete_authoritative_state() {
    assert_eq!(SNAPSHOT_STATE_FORMAT, "sim-core-bincode-v1");
    let world = world_with_active_voyage();
    let hash_before = world.stable_state_hash();

    let bytes = encode_world_snapshot(&world).unwrap();
    assert!(!bytes.is_empty());

    let restored = decode_world_snapshot(&bytes).unwrap();
    assert_eq!(restored, world);
    assert_eq!(restored.stable_state_hash(), hash_before);
}

#[test]
fn restored_world_continues_deterministically() {
    let mut control = world_with_active_voyage();
    let bytes = encode_world_snapshot(&control).unwrap();
    let mut restored = decode_world_snapshot(&bytes).unwrap();

    control.run_ticks(50);
    restored.run_ticks(50);

    assert_eq!(restored, control);
    assert_eq!(restored.stable_state_hash(), control.stable_state_hash());
}
