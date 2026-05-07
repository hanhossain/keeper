use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/admin/state", get(get_state));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct State {
    season: String,
    week: i32,
}

async fn get_state() -> Json<State> {
    tracing::info!("getting state");
    Json(State {
        season: String::from("2025"),
        week: 1,
    })
}
