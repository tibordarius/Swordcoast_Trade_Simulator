use world_data::SeedBundle;

#[test]
fn embedded_seed_loads_and_is_cross_referenced() {
    let bundle = SeedBundle::embedded_mvp().unwrap();
    assert_eq!(bundle.markets_len(), 6);
    assert_eq!(bundle.commodities_len(), 24);
    assert_eq!(bundle.states_for_market("MKT-WD").count(), 6);
    assert!(bundle.market("MKT-WD").is_some());
    assert!(bundle.commodity("CMD-GRAIN").is_some());
}

#[test]
fn market_states_are_sorted_deterministically() {
    let bundle = SeedBundle::embedded_mvp().unwrap();
    let keys: Vec<_> = bundle
        .market_states()
        .iter()
        .map(|state| (state.market_id.as_str(), state.commodity_id.as_str()))
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
}

#[test]
fn duplicate_market_state_is_rejected() {
    let markets = r#"{"markets":[{"id":"M","name":"Market","exchange":"MX"}]}"#;
    let commodities =
        r#"{"commodities":[{"id":"C","name":"Commodity","base_unit":"kg"}]}"#;
    let states = r#"{"states":[
      {"market_id":"M","commodity_id":"C","on_hand_milli":1,"target_reserve_milli":1,
       "reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,
       "liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0},
      {"market_id":"M","commodity_id":"C","on_hand_milli":1,"target_reserve_milli":1,
       "reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,
       "liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0}
    ]}"#;

    let error = SeedBundle::from_json(markets, commodities, states).unwrap_err();
    assert!(error.contains("duplicate market state"));
}
