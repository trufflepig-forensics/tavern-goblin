//! An extractor module for extracting the uuid of the user from the session

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use tower_sessions::Session;
use tracing::error;
use tracing::instrument;
use tracing::trace;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::SESSION_USER;
use crate::models::User;

/// The extractor for the uuid of the user from the session
pub struct SessionUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    #[instrument(level = "trace", skip_all)]
    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Ok(session) = Session::from_request_parts(req, state).await else {
            error!("Could not construct session");
            return Err(ApiError::InternalServerError);
        };
        let Some(user) = session.get::<Uuid>(SESSION_USER).await? else {
            trace!("{SESSION_USER} is missing in session");
            return Err(ApiError::Unauthenticated);
        };

        let user = query!(&GLOBAL.db, User)
            .condition(User::F.uuid.equals(user))
            .optional()
            .await?
            .ok_or(ApiError::Unauthenticated)?;

        Ok(SessionUser(user))
    }
}
