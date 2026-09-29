use tokio::net::TcpListener;
use wdex_api::{build_router, AppState};

#[tokio::main]
async fn main() {
    let bind = std::env::var("WDEX_BIND").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let world_id =
        std::env::var("WDEX_WORLD_ID").unwrap_or_else(|_| "WORLD-SWORD-COAST-V0".to_string());
    let seed = std::env::var("WDEX_WORLD_SEED")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(12_345);

    let listener = TcpListener::bind(&bind)
        .await
        .unwrap_or_else(|error| panic!("failed to bind {bind}: {error}"));

    axum::serve(listener, build_router(AppState::new(world_id, seed)))
        .await
        .expect("API server failed");
}
