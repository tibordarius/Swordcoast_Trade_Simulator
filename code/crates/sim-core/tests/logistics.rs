use sim_core::{InventoryLedger, Route, Shipment, ShipmentStatus};

const TICKS_PER_DAY: u64 = 288;

#[test]
fn reservation_departure_arrival_is_conservative_and_idempotent() {
    let route = Route {
        travel_ticks: 18 * TICKS_PER_DAY,
        capacity_per_day: 500_000_000,
        variable_cost_mcp_per_base_unit: 300,
        fixed_cost_mcp: 1_500_000,
    };
    let quantity = 100_000_000;
    assert!(route.accepts(quantity));
    assert_eq!(route.transport_cost_mcp(quantity), 31_500_000);

    let mut origin = InventoryLedger::new(1_000_000_000, 0, 700_000_000);
    let mut destination = InventoryLedger::new(720_000_000, 0, 1_200_000_000);
    assert!(origin.reserve(quantity));

    let departure = 100_000;
    let mut shipment = Shipment::new(1, quantity, departure, departure + route.travel_ticks);
    assert!(shipment.depart(&mut origin));
    assert_eq!(origin.on_hand, 900_000_000);
    assert_eq!(shipment.progress_bps(departure + route.travel_ticks / 2), 5_000);

    assert!(!shipment.arrive(&mut destination, shipment.eta_tick - 1));
    assert!(shipment.arrive(&mut destination, shipment.eta_tick));
    assert_eq!(destination.on_hand, 820_000_000);
    assert_eq!(shipment.status, ShipmentStatus::Arrived);

    assert!(!shipment.arrive(&mut destination, shipment.eta_tick + 100));
    assert_eq!(destination.on_hand, 820_000_000);
}

#[test]
fn route_capacity_can_reject_a_shipment() {
    let route = Route {
        travel_ticks: 100,
        capacity_per_day: 500_000_000,
        variable_cost_mcp_per_base_unit: 100,
        fixed_cost_mcp: 0,
    };
    assert!(!route.accepts(600_000_000));
}

#[test]
fn lost_shipment_never_reaches_destination() {
    let mut origin = InventoryLedger::new(200_000_000, 0, 100_000_000);
    let mut destination = InventoryLedger::new(0, 0, 100_000_000);
    let qty = 100_000_000;
    assert!(origin.reserve(qty));
    let mut shipment = Shipment::new(2, qty, 0, 100);
    assert!(shipment.depart(&mut origin));
    assert!(shipment.mark_lost());
    assert!(!shipment.arrive(&mut destination, 1000));
    assert_eq!(destination.on_hand, 0);
    assert_eq!(shipment.status, ShipmentStatus::Lost);
}
