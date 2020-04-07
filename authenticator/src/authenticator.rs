//! # Authenticator
//!
//! Describes the trait.

use serde::{Deserialize};
use http::response::Response;

/// Return codes for various functions
#[derive(PartialEq)]
pub enum AuthCodes {
    NotImplemented,
    DatabaseError,
    InternalError,
    UnregisteredUser,
    BadPassword,
    ChangedPassword,
    RegisterOk,
    LoginOk,
}

/// Data given from the Front-End
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

/// Bearer Token used for authentication
pub type Token = String;

/// Methods that any implementation needs to use
pub trait Authenticator {
    fn login(&self, user: UserCredentials) -> (Token, AuthCodes);
    fn register(&self, user: UserCredentials) -> AuthCodes;
    fn modify_password(&self, user: UserCredentials, new_password: String) -> AuthCodes;
}

/// Methods any controller needs to implement
pub trait AuthenticatorControl {
    fn login_response(&self, json: Option<&str>) -> Response;
    fn register_user_response(&self, json: Option<&str>) -> Response;
    fn modify_pass_response(&self, json: Option<&str>) -> Response;
}
