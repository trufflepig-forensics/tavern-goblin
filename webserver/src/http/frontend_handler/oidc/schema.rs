//! The schemas for oidc handlers

use openidconnect::core::CoreIdTokenClaims;
use openidconnect::core::CoreTokenResponse;
use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;
use openidconnect::Nonce;
use openidconnect::PkceCodeVerifier;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[allow(missing_docs)]
pub struct AuthState {
    pub csrf_token: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
    pub nonce: Nonce,
}

#[derive(Deserialize)]
#[allow(missing_docs)]
pub struct AuthRequest {
    pub code: AuthorizationCode,
    pub state: CsrfToken,
}

/// Data the [`super::handler::finish_login`] handler will store in the user's session
#[derive(Serialize, Deserialize)]
pub struct UserData {
    /// The oidc token
    pub token: CoreTokenResponse,

    /// The OIDC claims
    pub claims: CoreIdTokenClaims,
}
