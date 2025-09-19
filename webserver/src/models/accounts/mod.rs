//! Account model

use galvyn::core::re_exports::rorm;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::{ApiError, ApiResult};
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use std::ops::Deref;
use tracing::instrument;
use tracing::log::warn;
use uuid::Uuid;

use crate::models::accounts::db::AccountModel;

pub(in crate::models) mod db;

/// Account for user identification
pub struct Account {
    /// Primary key
    pub uuid: Uuid,

    /// The name that is used for displaying purposes
    pub display_name: MaxStr<255>,

    /// DN (distinguished name) for LDAP
    pub ldap_dn: MaxStr<2048>,

    /// Current balance of the user (i.e., what he/she owes to the community)
    pub balance: i64,

    /// Subject for OIDC
    pub sub: MaxStr<255>,
}

const SESSION_KEY: &str = "current_account_uuid";

impl Account {
    /// Update the display name of the current account
    #[instrument(name = "Account::set_display_name", skip(self, exe))]
    pub async fn set_display_name(
        &mut self,
        exe: impl Executor<'_>,
        display_name: MaxStr<255>,
    ) -> anyhow::Result<()> {
        rorm::update(exe, AccountModel)
            .set(AccountModel.display_name, display_name)
            .condition(AccountModel.uuid.equals(self.uuid))
            .await?;
        Ok(())
    }

    /// Find an account by its subject
    pub async fn find_by_subject(
        exe: impl Executor<'_>,
        sub: &MaxStr<255>,
    ) -> anyhow::Result<Option<Account>> {
        let account = rorm::query(exe, AccountModel)
            .condition(AccountModel.sub.equals(sub.deref()))
            .optional()
            .await?;
        Ok(account.map(Account::from))
    }

    pub async fn set_logged_in(&mut self, session: &Session) -> ApiResult<()> {
        session
            .insert(SESSION_KEY, self.uuid)
            .await
            .map_err(ApiError::map_server_error("Failed to write to session"))?;
        Ok(())
    }

    pub async fn set_logged_out(session: &Session) -> ApiResult<()> {
        if let Some(_account_uuid) = session.remove::<Uuid>(SESSION_KEY).await? {
            if let Some(_session_id) = session.id() {
                // TODO: notify websocket
            } else {
                warn!("A session with data should have an id!");
            }
        }
        Ok(())
    }
}

impl From<AccountModel> for Account {
    fn from(value: AccountModel) -> Self {
        Self {
            uuid: value.uuid,
            display_name: value.display_name,
            ldap_dn: value.ldap_dn,
            balance: value.balance,
            sub: value.sub,
        }
    }
}
