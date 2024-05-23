//! The schema for local authentication

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// The request for local authentication
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoginRequest {
    /// The username that is used for logging in
    pub username: String,
    /// The password for the user
    pub password: String,
}
