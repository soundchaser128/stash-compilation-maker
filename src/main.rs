use crate::ffmpeg::Ffmpeg;
use axum::{
    routing::{get, post},
    Router,
};
use std::{sync::Arc, time::Duration};

mod clip;
mod config;
mod download_ffmpeg;
mod error;
mod ffmpeg;
mod http;
mod stash_api;
mod static_files;
mod util;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct AppState {
    pub ffmpeg: Ffmpeg,
}

#[tokio::main]
async fn main() -> Result<()> {
    use std::env;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    config::init().await;

    let ffmpeg = Ffmpeg::new().await?;
    let state = Arc::new(AppState { ffmpeg });

    let app = Router::new()
        .route("/api/tags", get(http::fetch_tags))
        .route("/api/performers", get(http::fetch_performers))
        .route("/api/markers", get(http::fetch_markers))
        .route("/api/clips", get(http::fetch_clips))
        .route("/api/create", post(http::create_video))
        .route("/api/progress", get(http::get_progress))
        .route("/api/download/:id", get(http::download_video))
        .route("/api/config", get(http::get_config))
        .route("/api/config", post(http::set_config))
        .fallback_service(static_files::service())
        .with_state(state);

    let addr = "[::1]:5174";
    tracing::info!("running at {}", addr);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;

        if webbrowser::open("http://localhost:5174").is_err() {
            tracing::warn!(
                "failed to open UI in browser, please navigate to http://localhost:5147"
            );
        }
    });

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
