use scenario_pack_v2::{
    compile_initialization_commands, load_json, validate_pack, ScenarioPack,
    SCENARIO_PACK_SCHEMA_VERSION,
};
use sim_kernel_v2::{
    replay, state_hash, CommodityId, InventoryAccountId, MoneyAccountId, MoneyCp, Quantity,
    WorldState,
};

const TINY: &str = include_str!("fixtures/tiny_sword_coast.json");

fn raw_pack() -> ScenarioPack {
    serde_json::from_str(TINY).expect("fixture must parse as ScenarioPack")
}

fn issue_codes(error: scenario_pack_v2::ValidationError) -> Vec<String> {
    error.issues.into_iter().map(|issue| issue.code).collect()
}

#[test]
fn tiny_pack_loads_with_expected_shape() {
    let pack = load_json(TINY).unwrap();

    assert_eq!(pack.pack().manifest.schema_version, SCENARIO_PACK_SCHEMA_VERSION);
    assert_eq!(pack.pack().markets.len(), 3);
    assert_eq!(pack.pack().commodities.len(), 5);
    assert_eq!(pack.pack().routes.len(), 2);
    assert_eq!(pack.pack().places.len(), 3);
}

#[test]
fn tiny_pack_initializes_through_reducer_ledgers_deterministically() {
    let pack = load_json(TINY).unwrap();
    let commands = compile_initialization_commands(&pack);

    let initial = WorldState::new(pack.world_seed());
    let first = replay(&initial, &commands).unwrap();
    let second = replay(&initial, &commands).unwrap();

    assert_eq!(commands.len(), 10);
    assert_eq!(state_hash(&first).unwrap(), state_hash(&second).unwrap());
    assert_eq!(first, second);

    assert_eq!(
        first.inventory_balance(
            &InventoryAccountId::new("inventory.waterdeep"),
            &CommodityId::new("commodity.grain"),
        ),
        Some(Quantity::new(1000))
    );
    assert_eq!(
        first.inventory_balance(
            &InventoryAccountId::new("inventory.luskan"),
            &CommodityId::new("commodity.preserved_fish"),
        ),
        Some(Quantity::new(1200))
    );
    assert_eq!(
        first.money_balance(&MoneyAccountId::new("cash.neverwinter")),
        Some(MoneyCp::new(50000))
    );

    assert_eq!(first.inventory_ledger().len(), 30);
    assert_eq!(first.money_ledger().len(), 4);
    assert_eq!(first.revision().get(), commands.len() as u64);
}

#[test]
fn compilation_order_is_independent_of_account_and_opening_row_order() {
    let original = load_json(TINY).unwrap();
    let original_commands = compile_initialization_commands(&original);

    let mut shuffled = raw_pack();
    shuffled.inventory_accounts.reverse();
    shuffled.money_accounts.reverse();
    shuffled.opening_inventory.reverse();
    shuffled.opening_money.reverse();

    let shuffled = validate_pack(shuffled).unwrap();
    let shuffled_commands = compile_initialization_commands(&shuffled);

    assert_eq!(original_commands, shuffled_commands);
}

#[test]
fn unsupported_schema_version_is_rejected() {
    let mut pack = raw_pack();
    pack.manifest.schema_version = SCENARIO_PACK_SCHEMA_VERSION + 1;

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"unsupported_schema_version".to_owned()));
}

#[test]
fn duplicate_ids_are_rejected() {
    let mut pack = raw_pack();
    pack.commodities.push(pack.commodities[0].clone());

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"duplicate_id".to_owned()));
}

#[test]
fn dangling_unit_reference_is_rejected() {
    let mut pack = raw_pack();
    pack.commodities[0].base_unit = sim_kernel_v2::UnitId::new("unit.missing");

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"unknown_unit".to_owned()));
}

#[test]
fn invalid_unit_conversion_is_rejected() {
    let mut pack = raw_pack();
    pack.units[0].denominator = 0;

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"invalid_conversion".to_owned()));
}

#[test]
fn dangling_route_market_is_rejected() {
    let mut pack = raw_pack();
    pack.routes[0].to_market = sim_kernel_v2::MarketId::new("market.missing");

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"unknown_market".to_owned()));
}

#[test]
fn self_loop_and_zero_distance_route_are_rejected() {
    let mut pack = raw_pack();
    pack.routes[0].to_market = pack.routes[0].from_market.clone();
    pack.routes[0].distance_meters = 0;

    let error = validate_pack(pack).unwrap_err();
    let codes = issue_codes(error);

    assert!(codes.contains(&"route_self_loop".to_owned()));
    assert!(codes.contains(&"invalid_distance".to_owned()));
}

#[test]
fn dangling_opening_balance_account_is_rejected() {
    let mut pack = raw_pack();
    pack.opening_inventory[0].account_id =
        InventoryAccountId::new("inventory.missing");

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"unknown_inventory_account".to_owned()));
}

#[test]
fn reserved_system_account_ids_are_rejected() {
    let mut pack = raw_pack();
    pack.inventory_accounts[0].id =
        InventoryAccountId::new("system.opening.inventory");

    let error = validate_pack(pack).unwrap_err();
    assert!(issue_codes(error).contains(&"reserved_id".to_owned()));
}
