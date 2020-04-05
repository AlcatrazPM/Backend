//! # DataBase Connection Interface
//!
//! This module gets in depth with the `DataProvider` trait which all data providers
//! need to implement.
//!

use userdata::{siteinfo::SiteAccount, userinfo::Membership};

/// List of all possible return codes
pub enum DatabaseCodes {
    NotImplemented,
    // TODO: Add more errors
    OperationOK,
}

/// Basic methods of communication
pub trait DataProvider {
    fn add_site_account(&self, user_id: String, site: SiteAccount) -> DatabaseCodes;
    fn remove_site_account(&self, user_id: String, site: SiteAccount) -> DatabaseCodes;
    fn get_all_site_accounts(&self, ) -> Result<Vec<SiteAccount>, DatabaseCodes>;
    fn save_user_membership(&self, user_id: String, membership: Membership) -> DatabaseCodes;
    fn save_logs(&self, log: String) -> bool;
}
