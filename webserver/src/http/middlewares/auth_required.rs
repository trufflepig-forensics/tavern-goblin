//! Middleware which requires the user to be authenticated.

use std::ops::ControlFlow;

use galvyn::core::middleware::SimpleGalvynMiddleware;
use galvyn::core::re_exports::axum::extract::Request;
use galvyn::core::re_exports::axum::response::IntoResponse;
use galvyn::core::re_exports::axum::response::Response;

use crate::models::accounts::Account;

/// Middleware which requires the user to be authenticated.
#[derive(Copy, Clone, Debug)]
pub struct AuthRequiredLayer;

impl SimpleGalvynMiddleware for AuthRequiredLayer {
    async fn pre_handler(&mut self, req: Request) -> ControlFlow<Response, Request> {
        todo!();
        let (mut parts, body) = req.into_parts();
        /*match Account::from_request_parts(&mut parts, &()).await {
            Ok(_account) => ControlFlow::Continue(Request::from_parts(parts, body)),
            Err(rejection) => ControlFlow::Break(rejection.into_response()),
        }*/
    }
}
