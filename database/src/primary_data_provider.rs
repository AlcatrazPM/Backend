//! # Primary Data Provider
//!
//! Main implementation of the `DataProvider` trait.

use crate::data_provider::{DataProvider, DatabaseCodes};
use userdata::siteinfo::SiteAccount;
use userdata::userinfo::Membership;

pub struct PrimaryDataProvider;

impl PrimaryDataProvider {
    pub fn new() -> PrimaryDataProvider {
        PrimaryDataProvider
    }
}

impl DataProvider for PrimaryDataProvider {
    fn add_site_account(&self, user_id: String, site: SiteAccount) -> DatabaseCodes {
        DatabaseCodes::NotImplemented
    }

    fn remove_site_account(&self, user_id: String, site: SiteAccount) -> DatabaseCodes {
        DatabaseCodes::NotImplemented
    }

    fn get_all_site_accounts(&self, ) -> Result<Vec<SiteAccount>, DatabaseCodes> {
        Err(DatabaseCodes::NotImplemented)
    }
    fn save_user_membership(&self, user_id: String, membership: Membership) -> DatabaseCodes {
        DatabaseCodes::NotImplemented
    }
    fn save_logs(&self, log: String) -> bool {
        println!("This is a log: {}", log);
        false
    }
}
