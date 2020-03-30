//! # Accounts REST Controller
//!
//! This module implements the `AccountsProvider` trait.

use crate::accounts_provider::{AccountsCodes, AccountsProvider};
use database::data_provider::DataProvider;
use userdata::siteinfo::SiteAccount;

pub struct AccountsRestController<DP: DataProvider> {
    dataprovider: DP,
}

impl<DP> AccountsRestController<DP>
where
    DP: DataProvider,
{
    pub fn new(dp: DP) -> AccountsRestController<DP> {
        AccountsRestController { dataprovider: dp }
    }
}

impl<DP> AccountsProvider for AccountsRestController<DP>
where
    DP: DataProvider,
{
    fn add_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes {
        AccountsCodes::NotImplemented
    }

    fn remove_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes {
        AccountsCodes::NotImplemented
    }

    fn modify_site_account(
        &self,
        user_id: String,
        old_site_data: SiteAccount,
        new_site_data: SiteAccount,
    ) -> AccountsCodes {
        AccountsCodes::NotImplemented
    }

    fn get_all_site_accounts(&self) -> Result<Vec<SiteAccount>, AccountsCodes> {
        Err(AccountsCodes::NotImplemented)
    }
}
