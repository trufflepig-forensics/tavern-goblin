//! The handlers for authentication via openid-connect

use axum::extract::Query;
use axum::extract::State;
use axum::response::Redirect;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::reqwest::async_http_client;
use openidconnect::AccessTokenHash;
use openidconnect::CsrfToken;
use openidconnect::Nonce;
use openidconnect::OAuth2TokenResponse;
use openidconnect::PkceCodeChallenge;
use openidconnect::Scope;
use openidconnect::TokenResponse;
use rorm::insert;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use tower_sessions::Session;
use tracing::debug;
use tracing::instrument;
use tracing::trace;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::frontend_handler::oidc::schema::AuthRequest;
use crate::http::frontend_handler::oidc::schema::AuthState;
use crate::http::SESSION_OIDC_REQUEST;
use crate::http::SESSION_USER;
use crate::models::OidcUser;
use crate::models::User;
use crate::utils::oidc::OidcClient;

/// Handler for OIDC's login endpoint
#[instrument(skip_all, ret, level = "debug")]
pub async fn login(session: Session, client: State<OidcClient>) -> ApiResult<Redirect> {
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let request = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .set_pkce_challenge(pkce_code_challenge)
        .add_scope(Scope::new("profile".to_string()));
    let (auth_url, csrf_token, nonce) = request.url();

    // Store the csrf_token to verify it in finish_login
    session
        .insert(
            SESSION_OIDC_REQUEST,
            AuthState {
                csrf_token,
                pkce_code_verifier,
                nonce,
            },
        )
        .await?;

    Ok(Redirect::temporary(auth_url.as_str()))
}
/// Handler for the OIDC endpoint the user will be redirected to from the OIDC provider
#[instrument(skip_all, ret, level = "debug")]
pub async fn finish_login(
    client: State<OidcClient>,
    Query(AuthRequest { code, state }): Query<AuthRequest>,
    session: Session,
) -> ApiResult<Redirect> {
    // Get and remove the state generated in login
    let Some(AuthState {
        csrf_token,
        pkce_code_verifier,
        nonce,
    }) = session.remove(SESSION_OIDC_REQUEST).await?
    else {
        debug!("State is missing in key {SESSION_OIDC_REQUEST}");
        return Err(ApiError::Unauthenticated);
    };

    // Check the states to match
    if state.secret() != csrf_token.secret() {
        debug!("Secret state is invalid");
        return Err(ApiError::Unauthenticated);
    }

    // Exchange the code with a token.
    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .inspect_err(|e| debug!("Exchange code failed: {e}"))
        .map_err(|_| ApiError::Unauthenticated)?;

    // Extract the ID token claims after verifying its authenticity and nonce.
    let Some(id_token) = token.id_token() else {
        debug!("ID token is missing");
        return Err(ApiError::Unauthenticated);
    };
    let claims = id_token
        .claims(&client.id_token_verifier(), &nonce)
        .inspect_err(|e| debug!("ID token is invalid: {e}"))
        .map_err(|_| ApiError::Unauthenticated)?;

    // Verify the access token hash to ensure that the access token hasn't been substituted for
    // another user's.
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token.access_token(),
            &id_token
                .signing_alg()
                .inspect_err(|e| debug!("Retrieving signing alg failed: {e}"))
                .map_err(|_| ApiError::Unauthenticated)?,
        )
        .inspect_err(|e| debug!("Creating access token hash failed: {e}"))
        .map_err(|_| ApiError::Unauthenticated)?;
        if actual_access_token_hash != *expected_access_token_hash {
            debug!("The access token hash is invalid");
            return Err(ApiError::Unauthenticated);
        }
    }

    trace!("Got claims: {claims:#?}");

    let Some(username) = claims.preferred_username().map(|x| x.to_string()) else {
        debug!("Missing claim: preferred_username");
        return Err(ApiError::Unauthenticated);
    };

    let Some(claim) = claims.name() else {
        debug!("Missing claim: name");
        return Err(ApiError::Unauthenticated);
    };

    let Some(display_name) = claim.get(None).map(|x| x.to_string()) else {
        debug!("Missing localization for claim: name");
        return Err(ApiError::Unauthenticated);
    };

    let mut tx = GLOBAL.db.start_transaction().await?;

    let oidc_user = if let Some(user) = query!(&mut tx, OidcUser)
        .condition(OidcUser::F.oidc_id.equals(&username))
        .optional()
        .await?
    {
        user
    } else {
        let prim = insert!(&mut tx, User)
            .return_primary_key()
            .single(&User {
                uuid: Uuid::new_v4(),
                display_name,
            })
            .await?;

        insert!(&mut tx, OidcUser)
            .single(&OidcUser {
                uuid: Uuid::new_v4(),
                user: ForeignModelByField::Key(prim),
                oidc_id: username,
            })
            .await?
    };

    session.insert(SESSION_USER, *oidc_user.user.key()).await?;

    tx.commit().await?;

    Ok(Redirect::temporary("/"))
}
