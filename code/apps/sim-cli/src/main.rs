use sim_core::{StableStateHash, WorldState};

fn main() {
    let seed = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(12_345);
    let ticks = std::env::args()
        .nth(2)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(10_000);

    let mut world = WorldState::new(seed);
    world.run_ticks(ticks);

    println!("seed={seed}");
    println!("ticks={}", world.tick());
    println!("production_signal={}", world.production_signal());
    println!("state_hash={:016x}", world.stable_state_hash());
}
