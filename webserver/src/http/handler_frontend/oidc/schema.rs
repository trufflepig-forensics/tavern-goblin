use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::stuff::schema::SchemaString;
use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FinishOidcLoginRequest {
    pub code: SchemaString<AuthorizationCode>,
    pub state: SchemaString<CsrfToken>,
}
