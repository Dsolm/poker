/*!
  Module `http` exposes an HTTP server that handles HTTP requests to the application. Its
  implementation is opaque to module consumers.
*/
#[macro_use]
mod websocket;

use std::sync::Arc;

use anyhow::Context;
use axum::routing::get;
use tokio::net;

use crate::domain::blackjack::ports::BlackjackService;
use crate::inbound::http::websocket::blackjack::blackjack_ws_handler;

/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
pub struct AppState<BS: BlackjackService> {
    blackjack_service: Arc<BS>,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new<BS: BlackjackService>(
        blackjack_service: BS,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        // Construct dependencies to inject into handlers.
        let state = AppState {
            blackjack_service: Arc::new(blackjack_service),
        };

        let router = axum::Router::new()
            // .nest("/api", api_routes())
            .route(
                "/ws/blackjack",
                get(blackjack_ws_handler),
            )
            .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;

        Ok(())
    }
}
