use crate::{gameservers::SERVERS_LIMITED, metrics::Metrics};
use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};
use prometheus::{default_registry, Encoder, TextEncoder};
use std::{net::SocketAddr, sync::Arc};
use veloren_serverbrowser_api::GameServerList;

// Context for our reconciler
#[derive(Clone)]
pub struct Context {
    /// Prometheus metrics
    pub metrics: Metrics,
}

pub async fn server() {
    tracing::debug!("Starting up server");

    let context = Arc::new(Context {
        metrics: Metrics::new(),
    });

    // build our application with a route
    let app = Router::new()
        .route("/metrics", get(metrics))
        .route("/v1/servers", get(servers))
        .route("/health", get(health))
        .layer(Extension(context));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!(?addr, "listening");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    tracing::debug!("Shutdown server");
}

async fn metrics() -> Result<impl IntoResponse, StatusCode> {
    let metrics = default_registry().gather();
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&metrics, &mut buffer).unwrap();
    Ok(buffer)
}

async fn servers(Extension(context): Extension<Arc<Context>>) -> Json<GameServerList> {
    context
        .metrics
        .request
        .with_label_values(&["/v1/servers"])
        .inc();
    let _timer = context
        .metrics
        .request_duration
        .with_label_values(&["/v1/servers"])
        .start_timer();
    Json(SERVERS_LIMITED.clone())
}

async fn health() {}
