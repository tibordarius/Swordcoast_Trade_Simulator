use world_data::SeedBundle;

#[test]
fn embedded_seed_loads_complete_six_market_matrix() {
    let bundle = SeedBundle::embedded_mvp().unwrap();
    assert_eq!(bundle.markets_len(), 6);
    assert_eq!(bundle.commodities_len(), 24);
    assert_eq!(bundle.market_states().len(), 144);
    assert_eq!(bundle.routes_len(), 8);

    for market in ["MKT-WD", "MKT-BG", "MKT-ATH", "MKT-CAL", "MKT-NW", "MKT-LUS"] {
        assert_eq!(bundle.states_for_market(market).count(), 24);
    }

    assert_eq!(bundle.market("MKT-WD").unwrap().exchange, "WDEX");
    let grain = bundle.commodity("CMD-GRAIN").unwrap();
    assert_eq!(grain.provenance, "fr_canon");
    assert_eq!(grain.mass_grams_per_base_unit, 1_000);
    assert_eq!(grain.volume_cm3_per_base_unit, 1_300);

    let route = bundle.route("SEA-ATH-WD").unwrap();
    assert_eq!(route.travel_ticks, 5_184);
    assert_eq!(route.freight_mcp_per_kg, 315);
    assert_eq!(route.capacity_kg, 20_000_000);
}

#[test]
fn seed_order_does_not_change_loaded_bundle() {
    let markets_a = r#"{"markets":[
      {"id":"M2","name":"Two","exchange":"X2","liquidity_tier":2,"role":"r","provenance":"inference"},
      {"id":"M1","name":"One","exchange":"X1","liquidity_tier":1,"role":"r","provenance":"campaign_canon"}
    ]}"#;
    let markets_b = r#"{"markets":[
      {"id":"M1","name":"One","exchange":"X1","liquidity_tier":1,"role":"r","provenance":"campaign_canon"},
      {"id":"M2","name":"Two","exchange":"X2","liquidity_tier":2,"role":"r","provenance":"inference"}
    ]}"#;
    let commodities_a = r#"{"commodities":[
      {"id":"C2","name":"Two","base_unit":"kg","quantity_scale":1000,"family":"x","perishability":"none","fungibility":"full","reference_price_cp":2,"reference_price_status":"seed","provenance":"inference"},
      {"id":"C1","name":"One","base_unit":"kg","quantity_scale":1000,"family":"x","perishability":"none","fungibility":"full","reference_price_cp":1,"reference_price_status":"seed","provenance":"fr_canon"}
    ]}"#;
    let commodities_b = r#"{"commodities":[
      {"id":"C1","name":"One","base_unit":"kg","quantity_scale":1000,"family":"x","perishability":"none","fungibility":"full","reference_price_cp":1,"reference_price_status":"seed","provenance":"fr_canon"},
      {"id":"C2","name":"Two","base_unit":"kg","quantity_scale":1000,"family":"x","perishability":"none","fungibility":"full","reference_price_cp":2,"reference_price_status":"seed","provenance":"inference"}
    ]}"#;
    let states_a = r#"{"states":[
      {"market_id":"M2","commodity_id":"C2","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":2,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":2,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"},
      {"market_id":"M1","commodity_id":"C2","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":2,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"},
      {"market_id":"M2","commodity_id":"C1","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":2,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"},
      {"market_id":"M1","commodity_id":"C1","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"}
    ]}"#;
    let states_b = r#"{"states":[
      {"market_id":"M1","commodity_id":"C1","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"},
      {"market_id":"M2","commodity_id":"C1","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":2,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"},
      {"market_id":"M1","commodity_id":"C2","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":2,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"},
      {"market_id":"M2","commodity_id":"C2","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":2,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":2,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"}
    ]}"#;

    let a = SeedBundle::from_json(markets_a, commodities_a, states_a).unwrap();
    let b = SeedBundle::from_json(markets_b, commodities_b, states_b).unwrap();
    assert_eq!(a, b);
}

#[test]
fn duplicate_market_id_is_rejected() {
    let markets = r#"{"markets":[
      {"id":"M","name":"One","exchange":"X1","liquidity_tier":1,"role":"r","provenance":"inference"},
      {"id":"M","name":"Two","exchange":"X2","liquidity_tier":1,"role":"r","provenance":"inference"}
    ]}"#;
    let commodities = r#"{"commodities":[]}"#;
    let states = r#"{"states":[]}"#;
    let error = SeedBundle::from_json(markets, commodities, states).unwrap_err();
    assert!(error.contains("duplicate market id"));
}

#[test]
fn missing_cross_reference_is_rejected() {
    let markets = r#"{"markets":[
      {"id":"M","name":"Market","exchange":"MX","liquidity_tier":1,"role":"r","provenance":"inference"}
    ]}"#;
    let commodities = r#"{"commodities":[
      {"id":"C","name":"Commodity","base_unit":"kg","quantity_scale":1000,"family":"x","perishability":"none","fungibility":"full","reference_price_cp":1,"reference_price_status":"seed","provenance":"inference"}
    ]}"#;
    let states = r#"{"states":[
      {"market_id":"UNKNOWN","commodity_id":"C","on_hand_milli":1,"target_reserve_milli":1,"reference_price_mcp":1,"daily_supply_milli":0,"daily_demand_milli":1,"liquidity_tier":1,"depth_milli":1,"incoming_committed_milli":0,"risk_bps":0,"provenance":"inference"}
    ]}"#;
    let error = SeedBundle::from_json(markets, commodities, states).unwrap_err();
    assert!(error.contains("unknown market"));
}
