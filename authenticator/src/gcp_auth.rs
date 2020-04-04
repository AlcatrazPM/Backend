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
    fn login(&self, user: UserCredentials) -> (Token, AuthCodes) {
        // mock response
        if user.username == "vlad_e_hispter@gmail.com".to_string() {
            return (
                String::from("abCdeFGhi.JkLmNoPQRS.tuVWXyZ"),
                AuthCodes::LoginOk,
            );
        }

        (String::from(""), AuthCodes::UnregisteredUser)
        // end mock response
    }
    fn register(&self, user: UserCredentials) -> AuthCodes {
        // PrimaryDataProvider::save_logs("test".to_string());

        // mock response
        if user.username == "vlad_e_hispter@gmail.com".to_string() {
            return AuthCodes::RegisterOk;
        }
        return AuthCodes::InternalError;
        // end mock response
    }
    fn modify_password(&self, user: UserCredentials, new_password: String) -> AuthCodes {
        // mock response
        if user.password == "notarealpasswordjustthehash".to_string() {
            // voodoo magic password changed
            return AuthCodes::ChangedPassword;
        }
        // in case the old password was not correct
        AuthCodes::BadPassword
        // end mock response
    }
}
