use std::env;
use std::fs;
use std::process::ExitCode;

use scenario_pack_v2::{compile_initialization_commands, compile_registry, load_json};
use sim_kernel_v2::{replay, state_hash, WorldState};

fn main() -> ExitCode {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: scenario-pack-validate <scenario-pack.json>");
        return ExitCode::from(2);
    };

    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            return ExitCode::FAILURE;
        }
    };

    let validated = match load_json(&input) {
        Ok(pack) => pack,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let registry = match compile_registry(&validated) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("validated pack failed ScenarioRegistry compilation: {error:?}");
            return ExitCode::FAILURE;
        }
    };

    let commands = compile_initialization_commands(&validated);
    let initial = WorldState::new(validated.world_seed());
    let state = match replay(&initial, &commands) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("validated pack failed kernel initialization replay: {error:?}");
            return ExitCode::FAILURE;
        }
    };

    let hash = match state_hash(&state) {
        Ok(hash) => hash,
        Err(error) => {
            eprintln!("failed to hash initialized state: {error:?}");
            return ExitCode::FAILURE;
        }
    };

    println!(
        "valid pack={} revision={} units={} commodities={} places={} markets={} routes={} init_commands={} state_hash={hash:016x}",
        validated.pack().manifest.pack_id,
        validated.pack().manifest.revision,
        registry.units().len(),
        registry.commodities().len(),
        registry.places().len(),
        registry.markets().len(),
        registry.routes().len(),
        commands.len(),
    );

    ExitCode::SUCCESS
}
