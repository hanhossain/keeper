use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/admin/state", get(get_state));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct State {
    season: String,
    week: i32,
}

async fn get_state() -> Json<State> {
    Json(State {
        season: String::from("2025"),
        week: 1,
    })
}
