//! # Accounts REST Controller
//!
//! This module implements the `AccountsProvider` trait.

use crate::accounts_provider::{AccountsCodes, AccountsProvider};
use database::data_provider::DataProvider;
use userdata::siteinfo::SiteAccount;

pub struct AccountsRestController<'a, DP: DataProvider> {
    dataprovider: &'a DP,
}

impl<DP> AccountsRestController<'_, DP>
where
    DP: DataProvider,
{
    pub fn new<'a>(dataprovider: &'a DP) -> AccountsRestController<DP> {
        AccountsRestController { dataprovider }
    }
}

impl<DP> AccountsProvider for AccountsRestController<'_, DP>
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
