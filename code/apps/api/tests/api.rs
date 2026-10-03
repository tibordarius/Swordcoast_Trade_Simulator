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
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn every_mvp_exchange_is_seeded_with_all_commodities() {
    let app = build_router(AppState::new(WORLD, 12_345));
    for market in [
        "MKT-WD", "MKT-BG", "MKT-ATH", "MKT-CAL", "MKT-NW", "MKT-LUS",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/markets/{market}/snapshot"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{market}");
        let value = json_body(response).await;
        assert_eq!(
            value["commodities"].as_array().unwrap().len(),
            24,
            "{market}"
        );
        assert!(!value["exchange_code"].as_str().unwrap().is_empty());
    }
}

#[tokio::test]
async fn market_state_advances_from_seeded_supply_and_demand() {
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
    let grain = initial["commodities"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["commodity_id"] == "CMD-GRAIN")
        .unwrap();
    let initial_stock = grain["on_hand_milli"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let advance = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/worlds/{WORLD}/advance"))
                .header("content-type", "application/json")
                .body(Body::from(json!({"ticks":"288"}).to_string()))
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
    let grain = later["commodities"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["commodity_id"] == "CMD-GRAIN")
        .unwrap();
    let later_stock = grain["on_hand_milli"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    assert!(later_stock < initial_stock);
    assert_eq!(later["tick"], "288");
}

#[tokio::test]
async fn world_advance_is_serial_and_live_update_is_published() {
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
    assert_eq!(update.state_hash.len(), 16);
}

#[tokio::test]
async fn invalid_requests_are_rejected() {
    let app = build_router(AppState::new(WORLD, 12_345));
    let bad_ticks = app
        .clone()
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
    assert_eq!(bad_ticks.status(), StatusCode::BAD_REQUEST);

    let unknown = app
        .oneshot(
            Request::builder()
                .uri("/v1/markets/UNKNOWN/snapshot")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unknown.status(), StatusCode::NOT_FOUND);
}
