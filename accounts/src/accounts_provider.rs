//! # Accounts Provider
//!
//! The `AccountsProvider` trait is needed for any implementation of an account manager.

use userdata::siteinfo::SiteAccount;

/// List of all possible return codes
pub enum AccountsCodes {
    NotImplemented,
    // TODO: Add more codes
    OperationOK,
}

pub trait AccountsProvider {
    fn add_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes;
    fn remove_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes;
    fn modify_site_account(
        &self,
        user_id: String,
        old_site_data: SiteAccount,
        new_site_data: SiteAccount,
    ) -> AccountsCodes;
    fn get_all_site_accounts(&self) -> Result<Vec<SiteAccount>, AccountsCodes>;
}
