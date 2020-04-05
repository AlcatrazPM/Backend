//! # GCP Authentificator
//!
//! Implementation for GCP
use crate::authenticator::*;
// for logs
use database::data_provider::DataProvider;

pub struct GcpAuthenticator<'a, DP: DataProvider> {
    dataprovider: &'a DP,
}

impl<DP> GcpAuthenticator<'_, DP>
where
    DP: DataProvider,
{
    pub fn new(dataprovider: &DP) -> GcpAuthenticator<DP> {
        GcpAuthenticator {
            dataprovider,
        }
    }
}

impl<DP> Authenticator for GcpAuthenticator<'_, DP>
where
    DP: DataProvider,
{
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
            self.dataprovider.save_logs(format!("modified password to {}", new_password));
            return AuthCodes::ChangedPassword;
        }
        // in case the old password was not correct
        self.dataprovider.save_logs("wrong old password".to_string());
        AuthCodes::BadPassword
        // end mock response
    }
}
