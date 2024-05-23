//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use axum::routing::get;
use axum::Router;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPageBuilder;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::auth_required;
use crate::utils::oidc::OidcClient;

pub mod auth;
pub mod oidc;
pub mod users;

/// The Swagger definition for the frontend api v1
pub static FRONTEND_API_V1: SwaggapiPageBuilder =
    SwaggapiPageBuilder::new().filename("frontend_v1.json");

/// Create the router for the Frontend API
pub fn get_routes(oidc_client: OidcClient) -> Router {
    Router::new()
        .nest(
            "/api/frontend/v1/oidc",
            Router::new()
                .route("/login", get(oidc::handler::login))
                .route("/finish-login", get(oidc::handler::finish_login)),
        )
        .with_state(oidc_client)
        .merge(
            ApiContext::new("/api/frontend/v1/auth")
                .handler(auth::handler::login)
                .route_layer(ServiceBuilder::new().concurrency_limit(10)),
        )
        .merge(
            ApiContext::new("/api/frontend/v1")
                .handler(users::handler::get_me)
                .handler(auth::handler::logout)
                .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_required))),
        )
}
