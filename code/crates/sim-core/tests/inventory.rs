use sim_core::{ExactDailyRate, InventoryLedger};

const TICKS_PER_HOUR: u64 = 12;
const TICKS_PER_DAY: u64 = 288;

fn run_case(
    days: u64,
    start: i64,
    production_per_day: i64,
    consumption_per_day: i64,
    spoilage_ppm: i64,
) -> InventoryLedger {
    let mut ledger = InventoryLedger::new(start, 0, 700_000_000);
    let mut prod = ExactDailyRate::new(production_per_day);
    let mut cons = ExactDailyRate::new(consumption_per_day);
    let initial = start;

    for tick in 1..=(days * TICKS_PER_DAY) {
        if tick % TICKS_PER_HOUR == 0 {
            ledger.add_production(prod.next_hour());
            ledger.consume(cons.next_hour());
        }
        if tick % TICKS_PER_DAY == 0 {
            ledger.apply_spoilage_ppm(spoilage_ppm);
        }
    }

    assert_eq!(initial + ledger.produced, ledger.conservation_total());
    ledger
}

#[test]
fn normal_grain_case_matches_reference() {
    let ledger = run_case(30, 700_000_000, 30_000_000, 25_000_000, 500);
    assert_eq!(ledger.on_hand, 838_418_883);
    assert_eq!(ledger.produced, 900_000_000);
    assert_eq!(ledger.consumed, 750_000_000);
    assert_eq!(ledger.spoiled, 11_581_117);
    assert_eq!(ledger.unmet_demand, 0);
}

#[test]
fn shortage_records_unmet_demand_without_negative_stock() {
    let ledger = run_case(10, 50_000_000, 5_000_000, 25_000_000, 500);
    assert_eq!(ledger.on_hand, 0);
    assert_eq!(ledger.produced, 50_000_000);
    assert_eq!(ledger.consumed, 99_980_008);
    assert_eq!(ledger.spoiled, 19_992);
    assert_eq!(ledger.unmet_demand, 150_019_992);
    assert!(ledger.on_hand >= 0);
}

#[test]
fn hourly_rate_distribution_has_no_daily_drift() {
    let mut rate = ExactDailyRate::new(25_000_000);
    let mut total = 0;
    for _ in 0..24 {
        total += rate.next_hour();
    }
    assert_eq!(total, 25_000_000);
}
