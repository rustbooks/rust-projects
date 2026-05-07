mod api;
mod media;
mod models;
mod ws;
mod state;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,rustconf_backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data.db".to_string());
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("../migrations").run(&pool).await?;
    info!("Database migrations applied");

    let state = Arc::new(AppState::new(pool));

    let cors = CorsLayer::permissive(); // Tighten in production

    let app = Router::new()
        // REST: meeting lifecycle
        .route("/api/meetings", post(api::meetings::create_meeting))
        .route("/api/meetings/:id/join", post(api::meetings::join_meeting))
        .route("/api/meetings/:id", get(api::meetings::get_meeting))
        // Host controls (require host JWT)
        .route("/api/meetings/:id/mute/:peer_id", post(api::host::mute_peer))
        .route("/api/meetings/:id/unmute/:peer_id", post(api::host::unmute_peer))
        .route("/api/meetings/:id/kick/:peer_id", post(api::host::kick_peer))
        .route("/api/meetings/:id/promote/:peer_id", post(api::host::promote_peer))
        .route("/api/meetings/:id/close", post(api::host::close_meeting))
        // WebSocket signaling (SDP offer/answer, ICE candidates, chat, events)
        .route("/ws/:meeting_id/:peer_id", get(ws::handler::ws_handler))
        // WebRTC WHIP endpoint for str0m SFU
        .route("/whip/:meeting_id/:peer_id", post(media::whip::whip_offer))
        .route("/whip/:meeting_id/:peer_id", axum::routing::patch(media::whip::whip_ice))
        // Serve static frontend (dist/ built by trunk or plain HTML)
        .nest_service("/", ServeDir::new("../frontend/dist").fallback(
            axum::routing::get_service(
                tower_http::services::ServeFile::new("../frontend/dist/index.html")
            )
        ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("RustConf backend listening on {addr}");

    axum::serve(listener, app).await?;
    Ok(())
}
