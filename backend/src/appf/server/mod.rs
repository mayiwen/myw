use std::net::SocketAddr;

use crate::{appf::latency::LatencyOnResponse, config::server::ServerConfig};
use axum::{
    extract::{DefaultBodyLimit, Request},
    middleware, Extension, Router,
};
use bytesize::ByteSize;
use leptos::config::LeptosOptions;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{
    cors::{self, CorsLayer},
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
mod myw_middleware;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }
    pub async fn start(
        &self,
        state: LeptosOptions,
        router: Router<LeptosOptions>,
        db: DatabaseConnection,
    ) -> anyhow::Result<()> {
        let router = self.build_router(state, router, db);
        let port = self.config.port();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        // tracing::info!("listening on {}", listener.local_addr()?);
        tracing::info!("listening on http://localhost:{}", port);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }
    fn build_router(
        &self,
        state: LeptosOptions,
        router: Router<LeptosOptions>,
        db: DatabaseConnection,
    ) -> Router {
        let timeout = TimeoutLayer::new(Duration::from_secs(120));
        let body_limit = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));

        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                // let method = request.method().to_string();
                let method = request.method();

                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            // .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
            .on_response(LatencyOnResponse);
        let normalize_path = NormalizePathLayer::trim_trailing_slash();
        Router::new()
            .merge(router)
            .layer(super::middleware::get_auth_layer())
            .layer(Extension(db))
            // .layer(middleware::from_fn(myw_middleware::add_response_header::i))
            .layer(timeout)
            .layer(body_limit)
            // .layer(timeout)
            .layer(tracing)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
    }
}
