//! # Authenticator
//!
//! Describes the trait.

/// Return codes for various functions
pub enum AuthCodes {
    // TODO Add more Error
    NotImplemented,
    DatabaseError,
    UnregisteredUser,
    LoginOk,
}

/// Data given from the Front-End
pub struct UserCredentials {
    username: String,
    password: String,
    email: String,
}

use userdata::userinfo::User;

/// Methods that any implementation needs to use
pub trait Authenticator {
    fn login(&self, user: UserCredentials) -> Result<User, AuthCodes>;
    fn register(&self, user: UserCredentials) -> Result<User, AuthCodes>;
    fn modify_password(&self, user: User, password: String) -> bool;
}
