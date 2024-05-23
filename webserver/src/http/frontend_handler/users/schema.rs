//! The schema for the users

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// The full representation for the user
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullUser {
    /// The identifier of the user
    pub uuid: Uuid,
    /// Used for displaying purposes
    pub display_name: String,
}
