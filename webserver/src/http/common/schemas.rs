//! Common schemas in the API

use schemars::JsonSchema;
use schemars::JsonSchema_repr;
use serde::Deserialize;
use serde::Serialize;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
/// The Status code that are returned throughout the API
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, JsonSchema_repr)]
#[repr(u16)]
#[allow(missing_docs)]
pub enum ApiStatusCode {
    Unauthenticated = 1000,

    InternalServerError = 2000,
}

/// The response that is sent in a case of an error
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[allow(missing_docs)]
pub struct ApiErrorResponse {
    /// The Status code for the error.
    ///
    /// Important: Does not match http status codes
    pub status_code: ApiStatusCode,
    /// A human-readable error message.
    ///
    /// May be used for displaying purposes
    pub message: String,
}
