use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::Model;
use uuid::Uuid;

/// An account for logging into this admin platform
#[derive(Model, Clone, Debug)]
#[rorm(rename = "Account")]
pub struct AccountModel {
    /// Primary key
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The name that is used for displaying purposes
    pub display_name: MaxStr<255>,
}
