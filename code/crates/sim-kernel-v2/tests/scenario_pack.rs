use sim_kernel_v2::{
    state_hash, CommodityId, InventoryAccountId, InventoryAccountKind, MarketId, MoneyAccountId,
    MoneyCp, Quantity, RegistryError, ScenarioPack, ScenarioPackError, SCENARIO_PACK_SCHEMA_VERSION,
    SEED_INVENTORY_SOURCE_ACCOUNT, SEED_MONEY_EXTERNAL_ACCOUNT,
};

fn tiny_pack() -> ScenarioPack {
    ScenarioPack::from_json(include_str!("fixtures/tiny_scenario_pack.json"))
        .expect("tiny ScenarioPack fixture must parse")
}

#[test]
fn tiny_fixture_validates_and_loads() {
    let pack = tiny_pack();
    assert_eq!(pack.manifest.schema_version, SCENARIO_PACK_SCHEMA_VERSION);

    let loaded = pack.load().expect("tiny ScenarioPack must load");

    assert_eq!(loaded.registry().commodities().len(), 5);
    assert_eq!(loaded.registry().places().len(), 3);
    assert_eq!(loaded.registry().markets().len(), 3);
    assert_eq!(loaded.registry().routes().len(), 2);

    let world = loaded.world_state();
    assert_eq!(world.world_seed(), 424_242);
    assert_eq!(world.revision().get(), 17);
    assert_eq!(world.inventory_ledger().len(), 12);
    assert_eq!(world.money_ledger().len(), 6);

    assert_eq!(
        world.inventory_balance(
            &InventoryAccountId::new("inventory.waterdeep"),
            &CommodityId::new("commodity.grain"),
        ),
        Some(Quantity::new(1_000))
    );
    assert_eq!(
        world.money_balance(&MoneyAccountId::new("money.waterdeep.merchant")),
        Some(MoneyCp::new(100_000))
    );

    assert_eq!(
        world.inventory_balance(
            &InventoryAccountId::new(SEED_INVENTORY_SOURCE_ACCOUNT),
            &CommodityId::new("commodity.grain"),
        ),
        Some(Quantity::new(-1_500))
    );
    assert_eq!(
        world.money_balance(&MoneyAccountId::new(SEED_MONEY_EXTERNAL_ACCOUNT)),
        Some(MoneyCp::new(-225_000))
    );
}

#[test]
fn json_roundtrip_preserves_pack() {
    let pack = tiny_pack();
    let json = pack.to_json_pretty().unwrap();
    let decoded = ScenarioPack::from_json(&json).unwrap();
    assert_eq!(pack, decoded);
}

#[test]
fn same_semantics_with_reordered_arrays_produce_same_world_and_registry() {
    let original = tiny_pack();
    let mut reordered = original.clone();

    reordered.units.reverse();
    reordered.commodities.reverse();
    reordered.places.reverse();
    reordered.markets.reverse();
    reordered.routes.reverse();
    reordered.inventory_accounts.reverse();
    reordered.money_accounts.reverse();
    for account in &mut reordered.inventory_accounts {
        account.opening_balances.reverse();
    }

    let a = original.load().unwrap();
    let b = reordered.load().unwrap();

    assert_eq!(a.registry(), b.registry());
    assert_eq!(
        state_hash(a.world_state()).unwrap(),
        state_hash(b.world_state()).unwrap()
    );
    assert_eq!(a.world_state(), b.world_state());
}

#[test]
fn unsupported_schema_version_is_rejected() {
    let mut pack = tiny_pack();
    pack.manifest.schema_version = 999;

    assert_eq!(
        pack.validate(),
        Err(ScenarioPackError::UnsupportedSchemaVersion { found: 999 })
    );
}

#[test]
fn route_with_unknown_market_is_rejected() {
    let mut pack = tiny_pack();
    pack.routes[0].to_market = MarketId::new("market.missing");

    let error = pack.validate().unwrap_err();

    assert_eq!(
        error,
        ScenarioPackError::Registry(RegistryError::UnknownRouteMarket {
            route_id: pack.routes[0].id.clone(),
            market_id: MarketId::new("market.missing"),
        })
    );
}

#[test]
fn commodity_without_physical_dimensions_is_rejected() {
    let mut pack = tiny_pack();
    pack.commodities[0].mass_grams_per_base_unit = 0;

    let commodity_id = pack.commodities[0].id.clone();

    assert_eq!(
        pack.validate(),
        Err(ScenarioPackError::Registry(
            RegistryError::InvalidCommodityDimensions { commodity_id }
        ))
    );
}

#[test]
fn duplicate_inventory_account_is_rejected() {
    let mut pack = tiny_pack();
    pack.inventory_accounts.push(pack.inventory_accounts[0].clone());

    let account_id = pack.inventory_accounts[0].id.clone();

    assert_eq!(
        pack.validate(),
        Err(ScenarioPackError::DuplicateInventoryAccount(account_id))
    );
}

#[test]
fn negative_holding_opening_balance_is_rejected() {
    let mut pack = tiny_pack();
    pack.inventory_accounts[0].opening_balances[0].quantity = Quantity::new(-1);

    let account_id = pack.inventory_accounts[0].id.clone();
    let commodity_id = pack.inventory_accounts[0].opening_balances[0]
        .commodity_id
        .clone();

    assert_eq!(
        pack.validate(),
        Err(ScenarioPackError::NegativeOpeningInventory {
            account_id,
            commodity_id,
            quantity: Quantity::new(-1),
        })
    );
}

#[test]
fn source_or_sink_seed_account_is_reserved_for_loader() {
    let mut pack = tiny_pack();
    pack.inventory_accounts[0].id = InventoryAccountId::new(SEED_INVENTORY_SOURCE_ACCOUNT);
    pack.inventory_accounts[0].kind = InventoryAccountKind::SourceOrSink;

    assert_eq!(
        pack.validate(),
        Err(ScenarioPackError::ReservedInventoryAccount(
            InventoryAccountId::new(SEED_INVENTORY_SOURCE_ACCOUNT)
        ))
    );
}

#[test]
fn missing_status_in_json_is_not_silently_inferred() {
    let json = include_str!("fixtures/tiny_scenario_pack.json")
        .replacen(r#","status": "ReviewedMapping""#, "", 1);

    assert!(ScenarioPack::from_json(&json).is_err());
}
