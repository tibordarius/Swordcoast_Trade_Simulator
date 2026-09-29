use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sim_core::{StableStateHash, WorldState};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    world_id: Arc<str>,
    world: Arc<Mutex<WorldState>>,
}

impl AppState {
    pub fn new(world_id: impl Into<Arc<str>>, seed: u64) -> Self {
        Self {
            world_id: world_id.into(),
            world: Arc::new(Mutex::new(WorldState::new(seed))),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldStatus {
    pub world_id: String,
    pub seed: String,
    pub tick: String,
    pub production_signal: String,
    pub state_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct AdvanceRequest {
    pub ticks: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<ApiError>)>;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/worlds/{world_id}/status", get(status))
        .route("/v1/worlds/{world_id}/advance", post(advance))
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn status(
    State(state): State<AppState>,
    Path(world_id): Path<String>,
) -> ApiResult<WorldStatus> {
    ensure_world(&state, &world_id)?;
    let world = state.world.lock().await;
    Ok(Json(status_from_world(&state, &world)))
}

async fn advance(
    State(state): State<AppState>,
    Path(world_id): Path<String>,
    Json(request): Json<AdvanceRequest>,
) -> ApiResult<WorldStatus> {
    ensure_world(&state, &world_id)?;
    let ticks = request.ticks.parse::<u64>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                code: "invalid_ticks",
                message: "ticks must be an unsigned 64-bit decimal string".to_string(),
            }),
        )
    })?;

    let mut world = state.world.lock().await;
    world.run_ticks(ticks);
    Ok(Json(status_from_world(&state, &world)))
}

fn ensure_world(state: &AppState, requested: &str) -> Result<(), (StatusCode, Json<ApiError>)> {
    if requested == state.world_id.as_ref() {
        Ok(())
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                code: "world_not_found",
                message: format!("unknown world: {requested}"),
            }),
        ))
    }
}

fn status_from_world(state: &AppState, world: &WorldState) -> WorldStatus {
    WorldStatus {
        world_id: state.world_id.to_string(),
        seed: world.seed().to_string(),
        tick: world.tick().to_string(),
        production_signal: world.production_signal().to_string(),
        state_hash: format!("{:016x}", world.stable_state_hash()),
    }
}
