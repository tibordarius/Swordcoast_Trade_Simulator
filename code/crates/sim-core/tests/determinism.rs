use sim_core::{StableStateHash, WorldState};

const EXPECTED_10K_HASH: u64 = 0xea50aa8c6fbd16cb;
const EXPECTED_10K_SIGNAL: i64 = -19;
const EXPECTED_NEXT_SEQUENCE: u64 = 1668;

#[test]
fn same_seed_same_result() {
    let mut a = WorldState::new(12_345);
    let mut b = WorldState::new(12_345);
    a.run_ticks(10_000);
    b.run_ticks(10_000);
    assert_eq!(a, b);
    assert_eq!(a.stable_state_hash(), b.stable_state_hash());
}

#[test]
fn matches_reference_oracle() {
    let mut world = WorldState::new(12_345);
    world.run_ticks(10_000);
    assert_eq!(world.production_signal(), EXPECTED_10K_SIGNAL);
    assert_eq!(world.next_sequence(), EXPECTED_NEXT_SEQUENCE);
    assert_eq!(world.stable_state_hash(), EXPECTED_10K_HASH);
}

#[test]
fn different_seed_diverges() {
    let mut a = WorldState::new(12_345);
    let mut b = WorldState::new(54_321);
    a.run_ticks(10_000);
    b.run_ticks(10_000);
    assert_ne!(a.stable_state_hash(), b.stable_state_hash());
}

#[test]
fn unrelated_rng_stream_does_not_change_production() {
    let mut control = WorldState::new(12_345);
    let mut noisy = WorldState::new(12_345);
    for _ in 0..1_000 {
        noisy.draw_from_stream("events");
    }
    control.run_ticks(10_000);
    noisy.run_ticks(10_000);
    assert_eq!(control.production_signal(), noisy.production_signal());
    assert_ne!(control.stable_state_hash(), noisy.stable_state_hash());
}
