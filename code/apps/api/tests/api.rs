use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;
use wdex_api::{build_router, AppState};

const WORLD: &str = "WORLD-SWORD-COAST-V0";

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

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(value["tick"], "10000");
    assert_eq!(value["production_signal"], "-19");
    assert_eq!(value["state_hash"], "ea50aa8c6fbd16cb");

    let status = app
        .oneshot(
            Request::builder()
                .uri(format!("/v1/worlds/{WORLD}/status"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(status.into_body(), usize::MAX).await.unwrap();
    let value: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(value["tick"], "10000");
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
