//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use galvyn::core::GalvynRouter;
use galvyn::openapi::OpenapiRouterExt;

use crate::http::middlewares::auth_required::AuthRequiredLayer;

pub mod oidc;

/// Initialize the routes of the frontend
pub fn initialize_routes() -> GalvynRouter {
    let without_auth = GalvynRouter::new().nest(
        "/oidc",
        GalvynRouter::new().openapi_tag("OpenId Connect"),
        //.handler(oidc::handler::begin_oidc_login)
        //.handler(oidc::handler::finish_oidc_login)
    );

    let with_auth = GalvynRouter::new();

    without_auth.merge(with_auth.wrap(AuthRequiredLayer))
}
