use sim_core::{
    CalibratedRoute, CargoProfile, MarketCommodityKey, MarketCommodityState, ShipmentStatus,
    StableStateHash, TradeDispatchError, WorldState,
};

fn trade_world(capacity_kg: i64, travel_ticks: u64) -> WorldState {
    let mut world = WorldState::new(42);
    world.insert_cargo_profile("CMD-GRAIN", CargoProfile::new(1_000, 1_300));
    world.insert_route(CalibratedRoute::new(
        "SEA-ATH-WD",
        "MKT-ATH",
        "MKT-WD",
        true,
        travel_ticks,
        100,
        300,
        capacity_kg,
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
}

#[test]
fn dispatch_removes_origin_stock_and_arrival_changes_destination_later() {
    let mut world = trade_world(500_000, 2);
    let origin_before = world.market("MKT-ATH", "CMD-GRAIN").unwrap().on_hand_milli();
    let destination_before = world.market("MKT-WD", "CMD-GRAIN").unwrap().on_hand_milli();

    let shipment = world
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

    assert_eq!(shipment.status, ShipmentStatus::InTransit);
    assert_eq!(
        world.market("MKT-ATH", "CMD-GRAIN").unwrap().on_hand_milli(),
        origin_before - 100_000_000
    );
    assert_eq!(
        world.market("MKT-WD", "CMD-GRAIN").unwrap().on_hand_milli(),
        destination_before
    );
    assert_eq!(
        world
            .market("MKT-WD", "CMD-GRAIN")
            .unwrap()
            .incoming_committed_milli(),
        100_000_000
    );

    world.run_ticks(1);
    assert_eq!(world.shipment(shipment.id).unwrap().progress_bps(world.tick()), 5_000);
    assert_eq!(
        world.market("MKT-WD", "CMD-GRAIN").unwrap().on_hand_milli(),
        destination_before
    );

    world.run_ticks(1);
    assert_eq!(world.shipment(shipment.id).unwrap().status, ShipmentStatus::Arrived);
    assert_eq!(
        world.market("MKT-WD", "CMD-GRAIN").unwrap().on_hand_milli(),
        destination_before + 100_000_000
    );
    assert_eq!(
        world
            .market("MKT-WD", "CMD-GRAIN")
            .unwrap()
            .incoming_committed_milli(),
        0
    );
}

#[test]
fn aggregate_daily_lane_capacity_can_block_an_otherwise_profitable_shipment() {
    let mut world = trade_world(150_000, 1);
    world
        .dispatch_trade(
            "SEA-ATH-WD",
            "MKT-ATH",
            "MKT-WD",
            "CMD-GRAIN",
            100_000_000,
            500_000_000,
            0,
        )
        .unwrap();

    let error = world
        .dispatch_trade(
            "SEA-ATH-WD",
            "MKT-ATH",
            "MKT-WD",
            "CMD-GRAIN",
            100_000_000,
            500_000_000,
            0,
        )
        .unwrap_err();

    assert!(matches!(
        error,
        TradeDispatchError::RouteCapacityExceeded { .. }
    ));
}

#[test]
fn repeated_physical_arbitrage_reduces_the_trade_return_without_a_convergence_rule() {
    let mut world = trade_world(5_000_000, 1);
    let mut returns = Vec::new();

    for _ in 0..10 {
        match world.dispatch_trade(
            "SEA-ATH-WD",
            "MKT-ATH",
            "MKT-WD",
            "CMD-GRAIN",
            100_000_000,
            1_000_000_000,
            500,
        ) {
            Ok(shipment) => {
                let deployed = shipment.purchase_mcp + shipment.freight_mcp;
                returns.push((shipment.expected_profit_mcp() * 10_000) / deployed);
                world.run_ticks(1);
            }
            Err(TradeDispatchError::OpportunityRejected(_)) => break,
            Err(other) => panic!("unexpected dispatch result: {other:?}"),
        }
    }

    assert!(!returns.is_empty());
    assert!(returns.windows(2).all(|pair| pair[1] < pair[0]));
    assert!(returns.len() < 10);
}

#[test]
fn identical_trade_actions_replay_identically() {
    let mut a = trade_world(500_000, 2);
    let mut b = trade_world(500_000, 2);

    for world in [&mut a, &mut b] {
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
        world.run_ticks(2);
    }

    assert_eq!(a, b);
    assert_eq!(a.stable_state_hash(), b.stable_state_hash());
}
