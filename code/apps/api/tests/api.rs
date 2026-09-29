use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;
use wdex_api::{build_router, AppState};

const WORLD: &str = "WORLD-SWORD-COAST-V0";

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn health_is_ok() {
    let app = build_router(AppState::new(WORLD, 12_345));
    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn world_advance_is_serial_and_exact() {
    let app = build_router(AppState::new(WORLD, 12_345));

    let request = Request::builder()
        .method("POST")
        .uri(format!("/v1/worlds/{WORLD}/advance"))
        .header("content-type", "application/json")
        .body(Body::from(json!({"ticks":"10000"}).to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let value = json_body(response).await;
    assert_eq!(value["tick"], "10000");
    assert_eq!(value["production_signal"], "-19");
    assert_eq!(value["state_hash"].as_str().unwrap().len(), 16);

    let status = app
        .oneshot(
            Request::builder()
                .uri(format!("/v1/worlds/{WORLD}/status"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let value = json_body(status).await;
    assert_eq!(value["tick"], "10000");
}

#[tokio::test]
async fn wdex_grain_is_simulated_and_tightens_after_ten_days() {
    let app = build_router(AppState::new(WORLD, 12_345));

    let initial = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/markets/MKT-WD/snapshot")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let initial = json_body(initial).await;
    let initial_ask = initial["commodities"][0]["ask_mcp"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    assert_eq!(initial["commodities"][0]["on_hand_milli"], "3800000000");

    let advance = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/worlds/{WORLD}/advance"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ticks":"2880"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(advance.status(), StatusCode::OK);

    let later = app
        .oneshot(
            Request::builder()
                .uri("/v1/markets/MKT-WD/snapshot")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let later = json_body(later).await;
    let later_ask = later["commodities"][0]["ask_mcp"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    assert_eq!(later["tick"], "2880");
    assert_eq!(later["commodities"][0]["on_hand_milli"], "3600000000");
    assert!(later_ask > initial_ask);
}

#[tokio::test]
async fn advance_publishes_a_live_update() {
    let state = AppState::new(WORLD, 12_345);
    let mut updates = state.subscribe();
    let app = build_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/worlds/{WORLD}/advance"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ticks":"12"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let update = updates.recv().await.unwrap();
    assert_eq!(update.tick, "12");
}

#[tokio::test]
async fn invalid_tick_string_is_rejected() {
    let app = build_router(AppState::new(WORLD, 12_345));
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/worlds/{WORLD}/advance"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ticks":"1.5"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn unknown_world_is_not_found() {
    let app = build_router(AppState::new(WORLD, 12_345));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/worlds/UNKNOWN/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
