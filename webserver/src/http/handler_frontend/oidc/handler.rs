use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::re_exports::axum::response::Redirect;
use galvyn::core::re_exports::uuid::Uuid;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::get;
use galvyn::post;
use galvyn::rorm;
use galvyn::rorm::Database;
use galvyn::rorm::and;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::model::Identifiable;
use galvyn::rorm::prelude::ForeignModelByField;
use tracing::trace;

use crate::http::handler_frontend::oidc::schema::FinishOidcLoginRequest;
use crate::models::accounts::Account;
use crate::modules::oidc::OidcRequestState;
use crate::modules::oidc::OpenIdConnect;

#[get("/begin-login")]
pub async fn begin_oidc_login(session: Session) -> ApiResult<Redirect> {
    let (auth_url, session_state) = OpenIdConnect::global().begin_login()?;

    session.insert(SESSION_KEY, session_state).await?;

    Ok(Redirect::temporary(auth_url.as_str()))
}

#[get("/finish-login")]
pub async fn finish_oidc_login(
    session: Session,
    Query(request): Query<FinishOidcLoginRequest>,
) -> ApiResult<Redirect> {
    let session_state = session
        .remove(SESSION_KEY)
        .await?
        .ok_or(ApiError::bad_request(
            "There is no unfinished login challenge",
        ))?;

    let claims = OpenIdConnect::global()
        .finish_login(
            session_state,
            OidcRequestState {
                code: request.code.0,
                state: request.state.0,
            },
        )
        .await?;

    trace!(claims = serde_json::to_string(&claims).unwrap_or_else(|error| error.to_string()));

    let mut tx = Database::global().start_transaction().await?;

    // TODO: check issuer?

    let subject = MaxStr::new(claims.subject().to_string())
        .map_err(ApiError::map_server_error("Subject is too long"))?;

    let display_name = claims
        .name()
        .and_then(|localized| localized.get(None))
        .ok_or(ApiError::server_error(
            "Oidc provider did not provide the name claim",
        ))?;
    let display_name = MaxStr::new(display_name.to_string())
        .map_err(ApiError::map_server_error("Name is too long"))?;

    if let Some(mut account) = Account::find_by_subject(&mut tx, &subject).await? {
        // Account exists
        account.set_display_name(&mut tx, display_name).await?;
        account.set_logged_in(&session).await?;
    } else {
        // TODO: sync LDAP & create account
        // account.set_logged_in(&session).await?;
    };

    tx.commit().await?;

    Ok(Redirect::temporary("/"))
}

const SESSION_KEY: &str = "begin_oidc_login";

#[post("/logout")]
pub async fn logout(session: Session) -> ApiResult<()> {
    Account::set_logged_out(&session).await?;
    Ok(())
}
