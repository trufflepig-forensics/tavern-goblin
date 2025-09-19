//! Definitions of the configuration file

use std::net::{IpAddr, Ipv4Addr};
use std::sync::LazyLock;

use galvyn::core::stuff::env::{EnvError, EnvVar};
use galvyn::rorm::DatabaseDriver;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};

/// Load all environment variables declared in this module
///
/// Called at the beginning of `main` to gather and report all env errors at once.
pub fn load_env() -> Result<(), Vec<&'static EnvError>> {
    let mut errors = Vec::new();

    for result in [
        LISTEN_ADDRESS.load(),
        LISTEN_PORT.load(),
        OIDC_DISCOVER_URL.load(),
        OIDC_CLIENT_ID.load(),
        OIDC_CLIENT_SECRET.load(),
        OIDC_REDIRECT_URL.load(),
        POSTGRES_HOST.load(),
        POSTGRES_DB.load(),
        POSTGRES_PORT.load(),
        POSTGRES_USER.load(),
        POSTGRES_PASSWORD.load(),
    ] {
        errors.extend(result.err());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Address the API server should bind to
pub static LISTEN_ADDRESS: EnvVar<IpAddr> =
    EnvVar::optional("LISTEN_ADDRESS", || IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

/// Port the API server should bind to
pub static LISTEN_PORT: EnvVar<u16> = EnvVar::optional("LISTEN_PORT", || 8080);

/// The discovery url of internal oidc
pub static OIDC_DISCOVER_URL: EnvVar<IssuerUrl> = EnvVar::required("OIDC_DISCOVER_URL");

/// This server's client id for oidc
pub static OIDC_CLIENT_ID: EnvVar<ClientId> = EnvVar::required("OIDC_CLIENT_ID");

/// This server's client secret for oidc
pub static OIDC_CLIENT_SECRET: EnvVar<ClientSecret> = EnvVar::required("OIDC_CLIENT_SECRET");

/// The url the oidc servers should redirect the user to
pub static OIDC_REDIRECT_URL: EnvVar<RedirectUrl> = EnvVar::required("OIDC_REDIRECT_URL");

/// The address of the database server
pub static POSTGRES_HOST: EnvVar = EnvVar::optional("POSTGRES_HOST", || "postgres".to_string());

/// The database name
pub static POSTGRES_DB: EnvVar = EnvVar::required("POSTGRES_DB");

/// The port of the database server
pub static POSTGRES_PORT: EnvVar<u16> = EnvVar::optional("POSTGRES_PORT", || 5432);

/// The user to use for the database connection
pub static POSTGRES_USER: EnvVar = EnvVar::optional("POSTGRES_USER", || "postgres".to_string());

/// Password for the user
pub static POSTGRES_PASSWORD: EnvVar = EnvVar::optional("POSTGRES_PASSWORD", || "".to_string());

/// Bundle of all database variables combined in `rorm`'s format
pub static DB: LazyLock<DatabaseDriver> = LazyLock::new(|| DatabaseDriver::Postgres {
    name: POSTGRES_DB.clone(),
    host: POSTGRES_HOST.clone(),
    port: *POSTGRES_PORT,
    user: POSTGRES_USER.clone(),
    password: POSTGRES_PASSWORD.clone(),
});
