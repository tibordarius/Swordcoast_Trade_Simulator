use sim_core::{compute_price, market_buy};

#[test]
fn reserve_scarcity_is_monotonic() {
    let surplus = compute_price(2_000, 1_400_000_000, 700_000_000, 0, 25_000_000, 0, 0, 4);
    let normal = compute_price(2_000, 700_000_000, 700_000_000, 0, 25_000_000, 0, 0, 4);
    let shortage = compute_price(2_000, 350_000_000, 700_000_000, 0, 25_000_000, 0, 0, 4);
    assert_eq!(surplus.fundamental_mcp, 1_000);
    assert_eq!(normal.fundamental_mcp, 2_000);
    assert_eq!(shortage.fundamental_mcp, 3_500);
    assert!(surplus.fundamental_mcp < normal.fundamental_mcp);
    assert!(normal.fundamental_mcp < shortage.fundamental_mcp);
}

#[test]
fn incoming_supply_reduces_current_fundamental() {
    let none = compute_price(2_000, 350_000_000, 700_000_000, 0, 25_000_000, 0, 0, 4);
    let incoming = compute_price(2_000, 350_000_000, 700_000_000, 0, 25_000_000, 350_000_000, 0, 4);
    assert!(incoming.fundamental_mcp < none.fundamental_mcp);
}

#[test]
fn larger_market_order_has_worse_average_execution() {
    let normal = compute_price(2_000, 700_000_000, 700_000_000, 0, 25_000_000, 0, 0, 4);
    let small = market_buy(10_000_000, normal.ask_mcp, 40_000_000);
    let large = market_buy(100_000_000, normal.ask_mcp, 40_000_000);
    assert_eq!(small.average_price_mcp, 2_013);
    assert_eq!(large.average_price_mcp, 2_058);
    assert!(large.average_price_mcp > small.average_price_mcp);
}
