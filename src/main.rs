mod api;
mod app;
mod utils;

use axum::{routing::get, Router};

use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::websocket::ws_handler;
use app::App;

use crate::{
    api::rest::{get_clients, ping_client},
    utils::load_env,
};

#[tokio::main]
async fn main() {
    let config = load_env().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let app = Router::new()
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", get(ws_handler))
        .route("/clients", get(get_clients))
        .route("/ping_client/:client", get(ping_client))
        .with_state(App::default());

    let addr: SocketAddr = format!("0.0.0.0:{}", config.port)
        .parse()
        .expect("Can not parse address and port");

    tracing::info!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
