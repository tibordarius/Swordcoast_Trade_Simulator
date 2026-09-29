use sim_core::{MarketCommodityKey, MarketCommodityState, StableStateHash, WorldState};

fn wdex_grain() -> MarketCommodityState {
    MarketCommodityState::new(
        3_800_000_000,
        4_000_000_000,
        2_000,
        400_000_000,
        420_000_000,
        5,
        1_600_000_000,
        0,
        0,
    )
}

#[test]
fn market_supply_and_demand_are_exact_over_ten_days() {
    let mut world = WorldState::new(12_345);
    world.insert_market(
        MarketCommodityKey::new("MKT-WD", "CMD-GRAIN"),
        wdex_grain(),
    );

    let initial = world.market("MKT-WD", "CMD-GRAIN").unwrap();
    let initial_ask = initial.quote().ask_mcp;
    assert_eq!(initial.on_hand_milli(), 3_800_000_000);

    world.run_ticks(2_880);

    let grain = world.market("MKT-WD", "CMD-GRAIN").unwrap();
    assert_eq!(grain.on_hand_milli(), 3_600_000_000);
    assert_eq!(grain.volume_milli(), 4_200_000_000);
    assert!(grain.quote().ask_mcp > initial_ask);
}

#[test]
fn market_state_participates_in_world_hash() {
    let mut a = WorldState::new(12_345);
    let mut b = WorldState::new(12_345);
    a.insert_market(
        MarketCommodityKey::new("MKT-WD", "CMD-GRAIN"),
        wdex_grain(),
    );
    b.insert_market(
        MarketCommodityKey::new("MKT-WD", "CMD-GRAIN"),
        wdex_grain(),
    );

    assert_eq!(a.stable_state_hash(), b.stable_state_hash());
    b.run_ticks(12);
    assert_ne!(a.stable_state_hash(), b.stable_state_hash());
}

#[test]
fn marketless_world_preserves_original_reference_hash() {
    let mut world = WorldState::new(12_345);
    world.run_ticks(10_000);
    assert_eq!(world.stable_state_hash(), 0xea50aa8c6fbd16cb);
}
