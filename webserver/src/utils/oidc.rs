//! The OIDC client code is defined in this module

use std::ops::Deref;

use openidconnect::core::CoreClient;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::reqwest::async_http_client;
use openidconnect::reqwest::HttpClientError;
use openidconnect::DiscoveryError;

use crate::config::OpenIdConnect;

/// Client the [`handler`] depend on
#[derive(Clone)]
pub struct OidcClient {
    pub(crate) client: CoreClient,
}

impl OidcClient {
    /// Use the discover endpoints of the provided OIDC server and return a client
    pub async fn discover(conf: OpenIdConnect) -> Result<Self, DiscoveryError<HttpClientError>> {
        let OpenIdConnect {
            client_id,
            client_secret,
            discover_url,
            redirect_url,
        } = conf;

        let provider_metadata =
            CoreProviderMetadata::discover_async(discover_url, async_http_client).await?;
        let client =
            CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
                .set_redirect_uri(redirect_url);

        Ok(OidcClient { client })
    }
}

impl Deref for OidcClient {
    type Target = CoreClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
