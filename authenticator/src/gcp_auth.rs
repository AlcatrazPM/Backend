//! # GCP Authentificator
//!
//! Implementation for GCP
use crate::authenticator::*;
use userdata::userinfo::User;
// for logs
use database::data_provider::DataProvider;
use database::primary_data_provider::PrimaryDataProvider;

pub struct GcpAuthenticator;

impl GcpAuthenticator {
    pub fn new() -> GcpAuthenticator {
        GcpAuthenticator
    }
}

impl Authenticator for GcpAuthenticator {
    fn login(&self, user: UserCredentials) -> Result<User, AuthCodes> {
        Err(AuthCodes::NotImplemented)
    }
    fn register(&self, user: UserCredentials) -> Result<User, AuthCodes> {
        // PrimaryDataProvider::save_logs("test".to_string());
        Err(AuthCodes::NotImplemented)
    }
    fn modify_password(&self, user: User, password: String) -> bool {
        false
    }
}
