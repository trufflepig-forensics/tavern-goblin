//! Definitions of the configuration file

use galvyn::core::stuff::env::{EnvError, EnvVar};
use galvyn::rorm::DatabaseDriver;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::LazyLock;

/// Load all environment variables declared in this module
///
/// Called at the beginning of `main` to gather and report all env errors at once.
pub fn load_env() -> Result<(), Vec<&'static EnvError>> {
    let mut errors = Vec::new();

    for result in [
        LISTEN_ADDRESS.load(),
        LISTEN_PORT.load(),
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
