//! The http part of the webserver

use galvyn::core::{GalvynRouter, SchemalessJson};
use galvyn::get;
use galvyn::openapi::{OpenAPI, OpenapiRouterExt};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, instrument};

pub mod handler_frontend;
pub mod middlewares;

/// The key for accessing the user in the session
pub const SESSION_USER: &str = "user";
/// The key for accessing and storing the data required for a secure OIDC request
///
/// I.e. csrf token, some nonce, etc.
pub const SESSION_OIDC_REQUEST: &str = "oidc_request";

pub struct FrontendApi;

#[get("/openapi.json")]
#[instrument]
async fn get_openapi() -> SchemalessJson<&'static OpenAPI> {
    SchemalessJson(galvyn::openapi::get_openapi())
}

/// Openapi page for the API
pub struct Api;

/// Initialize the router
pub fn initialize_routes() -> GalvynRouter {
    GalvynRouter::new()
        .nest("/docs", {
            GalvynRouter::new()
                .openapi_tag("Openapi")
                .handler(get_openapi)
        })
        .nest(
            "/api/frontend/v1",
            handler_frontend::initialize_routes().openapi_page(FrontendApi),
        )
        //GalvynRouter::with_openapi_page(Api).layer(axum::middleware::from_fn(middlewares::auth_required::auth_required))
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO)))
}
