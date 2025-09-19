use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::{Model, Patch};
use uuid::Uuid;

/// An account for logging into this admin platform
#[derive(Debug, Model)]
#[rorm(rename = "Account")]
pub struct AccountModel {
    /// Primary key
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The name that is used for displaying purposes
    pub display_name: MaxStr<255>,

    /// DN (distinguished name) for LDAP
    #[rorm(unique)]
    pub ldap_dn: MaxStr<2048>,

    /// Current balance of the user (i.e., what he/she owes to the community)
    pub balance: i64,

    /// Subject for OIDC
    #[rorm(unique)]
    pub sub: MaxStr<255>,
}

#[derive(Debug, Patch)]
#[rorm(model = "AccountModel")]
pub struct AccountModelInsert {
    pub uuid: Uuid,
    pub display_name: MaxStr<255>,
    pub ldap_dn: MaxStr<2048>,
    pub balance: i64,
    pub sub: MaxStr<255>,
}
