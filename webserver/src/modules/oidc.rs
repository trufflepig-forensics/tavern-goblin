//! Galvyn [`Module`] containing the state and logic for OpenID Connect authentication.

use std::time::Duration;

use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use openidconnect::AccessTokenHash;
use openidconnect::AuthorizationCode;
use openidconnect::ClientId;
use openidconnect::ClientSecret;
use openidconnect::CsrfToken;
use openidconnect::DiscoveryError;
use openidconnect::EndpointMaybeSet;
use openidconnect::EndpointNotSet;
use openidconnect::EndpointSet;
use openidconnect::HttpClientError;
use openidconnect::IssuerUrl;
use openidconnect::Nonce;
use openidconnect::OAuth2TokenResponse;
use openidconnect::PkceCodeChallenge;
use openidconnect::PkceCodeVerifier;
use openidconnect::RedirectUrl;
use openidconnect::RequestTokenError;
use openidconnect::Scope;
use openidconnect::TokenResponse;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::core::CoreClient;
use openidconnect::core::CoreIdTokenClaims;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::reqwest;
use serde::Deserialize;
use serde::Serialize;
use tracing::error;
use tracing::warn;
use url::Url;

use crate::config::OIDC_CLIENT_ID;
use crate::config::OIDC_CLIENT_SECRET;
use crate::config::OIDC_DISCOVER_URL;
use crate::config::OIDC_REDIRECT_URL;

/// Galvyn [`Module`] containing the state and logic for OpenID Connect authentication.
pub struct OpenIdConnect {
    /// HTTP client used to send requests to the issuer.
    ///
    /// It is stored as a field to make efficient use of resources.
    /// Most noteworthy, a connection pool.
    http_client: reqwest::Client,

    /// OIDC client used to construct requests for the issuer.
    ///
    /// This type mostly consists of configuration and logic.
    oidc_client: OidcClient,
}

/// Type alias for the highly generic [`CoreClient`] type.
type OidcClient = CoreClient<
    EndpointSet, // Auth URL
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet, // Token URL
    EndpointMaybeSet,
>;

/// The part of the state required during an ongoing oidc authentication which is stored in the user's session.
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcSessionState {
    /// Random value which takes a round trip through the issuer to detect tampering.
    pub csrf_token: CsrfToken,

    /// Random one-time secret which proves to the issuer that the two oauth requests are coming from the same party.
    pub pkce_code_verifier: PkceCodeVerifier,

    /// Nonce used to verify the id token's integrity.
    pub nonce: Nonce,
}

/// The part of the state required during an ongoing oidc authentication which is passed in the redirect url.
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcRequestState {
    /// Code which can be exchanged by the issuer for an id token.
    pub code: AuthorizationCode,

    /// Random value which took a round trip through the issuer to detect tampering.
    pub state: CsrfToken,
}

impl OpenIdConnect {
    /// Initiates a new oidc authentication.
    ///
    /// # Returns
    /// The url the user should be redirected to
    /// and some state that should be stored in the user's session.
    pub fn begin_login(&self) -> ApiResult<(Url, OidcSessionState)> {
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let request = self
            .oidc_client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .set_pkce_challenge(pkce_code_challenge)
            .add_scope(Scope::new("profile".to_string()));
        let (auth_url, csrf_token, nonce) = request.url();

        Ok((
            auth_url,
            OidcSessionState {
                csrf_token,
                nonce,
                pkce_code_verifier,
            },
        ))
    }

    /// Finishes an ongoing oidc authentication
    ///
    /// This method is called when the user is redirected back to us from the issuer.
    ///
    /// # Arguments
    /// - `session` the state returned by the associated [`Self::begin_login`] call
    /// - `request` the state passed from the issuer through the redirect url.
    ///
    /// # Returns
    /// The claims provided by the issuer.
    ///
    /// The caller is responsible for processing them into database models.
    pub async fn finish_login(
        &self,
        session: OidcSessionState,
        request: OidcRequestState,
    ) -> ApiResult<CoreIdTokenClaims> {
        // Check the states to match
        if request.state != session.csrf_token {
            return Err(ApiError::unauthorized("Secret state is invalid"));
        }

        // Exchange the authorization code with a token.
        let token_response = self
            .oidc_client
            .exchange_code(request.code)
            .set_pkce_verifier(session.pkce_code_verifier)
            .request_async(&self.http_client)
            .await
            .map_err(|error| match error {
                RequestTokenError::ServerResponse(_) => {
                    ApiError::server_error("server response invalid")
                }
                _ => ApiError::bad_request("bad request"),
            })?;

        // Extract the ID token claims after verifying its authenticity and nonce.
        let id_token = token_response.id_token().ok_or(ApiError::server_error(
            "Oidc provider did not provider an id token. \
            This would suggest its not providing oidc.",
        ))?;
        let id_token_verifier = self.oidc_client.id_token_verifier();
        let claims = id_token
            .claims(&id_token_verifier, &session.nonce)
            .map_err(|error| ApiError::unauthorized("Failed to verify id token"))?;

        // Verify the access token hash to ensure that the access token hasn't been substituted for
        // another user's.
        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                id_token.signing_alg().map_err(ApiError::map_server_error(
                    "Failed to retrieve signing algorithm",
                ))?,
                id_token
                    .signing_key(&id_token_verifier)
                    .map_err(ApiError::map_server_error("Failed to retrieve signing key"))?,
            )
            .map_err(ApiError::map_server_error(
                "Failed to recreate access token signature",
            ))?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(ApiError::unauthorized("Invalid access token"));
            }
        }

        Ok(claims.clone())
    }
}

impl Module for OpenIdConnect {
    type Setup = ();
    type PreInit = Self;

    async fn pre_init((): Self::Setup) -> Result<Self::PreInit, PreInitError> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        let oidc_client = OidcConfig::from_env()
            .discover_retry::<3>(&http_client)
            .await?;

        Ok(Self {
            http_client,
            oidc_client,
        })
    }

    type Dependencies = ();

    async fn init(pre_init: Self::PreInit, (): &mut Self::Dependencies) -> Result<Self, InitError> {
        Ok(pre_init)
    }
}

/// Config set by admin to connect to the oidc provider.
struct OidcConfig {
    /// The provider's url.
    url: IssuerUrl,

    /// The client id
    client_id: ClientId,

    /// The client secret
    client_secret: ClientSecret,

    /// The url to redirect users to, to finish the authentication.
    redirect_url: RedirectUrl,
}

impl OidcConfig {
    /// Loads the config from environment variables
    fn from_env() -> Self {
        OidcConfig {
            url: OIDC_DISCOVER_URL.clone(),
            client_id: OIDC_CLIENT_ID.clone(),
            client_secret: OIDC_CLIENT_SECRET.clone(),
            redirect_url: OIDC_REDIRECT_URL.clone(),
        }
    }

    /// Tries to discover the provider's configuration `N` times
    async fn discover_retry<const N: usize>(
        &self,
        http_client: &reqwest::Client,
    ) -> Result<OidcClient, DiscoveryError<HttpClientError<reqwest::Error>>> {
        let mut result = Err(DiscoveryError::Other(String::new()));
        for _ in 0..N {
            result = self.discover(http_client).await;
            if let Err(DiscoveryError::Request(HttpClientError::Reqwest(error))) = &result {
                #[allow(clippy::collapsible_if, reason = "Feature to new")]
                if error.is_timeout() {
                    warn!("Timed out fetching oidc discovery, trying again...");
                    continue;
                }
            }
            return result;
        }
        error!("Timed out fetching oidc discovery");
        result
    }

    /// Tries to discover the provider's configuration
    async fn discover(
        &self,
        http_client: &reqwest::Client,
    ) -> Result<OidcClient, DiscoveryError<HttpClientError<reqwest::Error>>> {
        let oidc_client = CoreClient::from_provider_metadata(
            CoreProviderMetadata::discover_async(self.url.clone(), http_client).await?,
            self.client_id.clone(),
            Some(self.client_secret.clone()),
        )
        .set_redirect_uri(self.redirect_url.clone());

        // Check the token url to be set
        let token_uri = oidc_client
            .token_uri()
            .ok_or_else(|| DiscoveryError::Other("Issuer did not provide a token url".to_string()))?
            .clone();
        let oidc_client = oidc_client.set_token_uri(token_uri);

        Ok(oidc_client)
    }
}
