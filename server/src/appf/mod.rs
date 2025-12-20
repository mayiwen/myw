pub mod database;

pub mod error;
pub mod latency;
pub mod logger;
pub mod response;

pub mod auth;
pub mod common;
pub mod json;
pub mod middleware;
pub mod path;
pub mod query;
pub mod serde;
pub mod server;
pub mod utils;
pub mod valid;
pub mod validdation;
use axum::Router;
use sea_orm::DatabaseConnection;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting app server...");
    let db = database::init().await?;
    let state = AppState::new(db);
    let server = server::Server::new(crate::config::get().server());
    server.start(state, router).await
}
