//! # Accounts Provider
//!
//! The `AccountsProvider` trait is needed for any implementation of an account manager.

use http::response::Response;
use userdata::siteinfo::SiteAccount;

/// List of all possible return codes
#[derive(PartialEq)]
pub enum AccountsCodes {
    NotImplemented,
    // TODO: Add more codes
    OperationOK,
}

pub trait AccountsProvider {
    fn add_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes;
    fn remove_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes;
    fn modify_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes;
    fn get_all_site_accounts(&self, user_id: String) -> (Vec<SiteAccount>, AccountsCodes);
}

pub trait AccountsControl {
    fn modify_site_account_response(&self, json: Option<&str>, jwt: &str) -> Response;
    fn get_all_site_accounts_response(&self, jwt: &str) -> Response;
}
