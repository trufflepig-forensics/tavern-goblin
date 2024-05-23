//! Configuration and start of the webserver is defined in this module

use std::io;
use std::net::AddrParseError;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;

use axum::Router;
use futures::StreamExt;
use openidconnect::reqwest::HttpClientError;
use openidconnect::DiscoveryError;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook_tokio::Signals;
use swaggapi::SwaggerUi;
use thiserror::Error;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::SameSite;
use tower_sessions::Expiry;
use tower_sessions::SessionManagerLayer;
use tower_sessions_rorm_store::RormStore;
use tracing::error;
use tracing::info;
use tracing::info_span;
use tracing::instrument;
use tracing::Instrument;

use crate::config::Config;
use crate::global::GLOBAL;
use crate::http::frontend_handler;
use crate::http::frontend_handler::FRONTEND_API_V1;
use crate::models;
use crate::utils::oidc::OidcClient;

/// Start the http server
#[instrument(skip_all, ret)]
pub async fn run(config: &Config) -> Result<(), StartServerError> {
    let oidc_client = OidcClient::discover(config.openid_connect.clone()).await?;
    info!("OIDC connected successfully!");

    let router = Router::new()
        .merge(frontend_handler::get_routes(oidc_client))
        .merge(SwaggerUi::with_path("/docs").page("Frontend API", &FRONTEND_API_V1))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    SessionManagerLayer::new(RormStore::<models::Session>::new(GLOBAL.db.clone()))
                        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)))
                        .with_same_site(SameSite::Lax),
                ),
        );

    let socket_addr = SocketAddr::new(
        IpAddr::from_str(&config.server.listen_address)?,
        config.server.listen_port,
    );

    info!("Start to listen on http://{socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(handle_signals().instrument(info_span!("signals")))
        .await?;

    Ok(())
}

async fn handle_signals() {
    let Ok(mut signals) = Signals::new(TERM_SIGNALS) else {
        error!("Could not register signals");
        return;
    };
    let handle = signals.handle();

    if let Some(signal) = signals.next().await {
        info!("Received signal {signal}, exiting ..");
    }

    handle.close();
}

/// Errors that can occur while starting the server
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum StartServerError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] AddrParseError),
    #[error("Connection to oidc failed: {0}")]
    OidcConnectionFailed(#[from] DiscoveryError<HttpClientError>),
}
