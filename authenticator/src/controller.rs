//! # Authentication Rest Controller
//!
//! Used to execute any all actions needed for authentication

use crate::authenticator::{AuthCodes, Authenticator};

/// Authentication Controller
pub struct AuthRestController<A: Authenticator> {
    auth: A,
}

impl<A> AuthRestController<A>
where
    A: Authenticator,
{
    pub fn new(auth: A) -> AuthRestController<A> {
        AuthRestController { auth }
    }

    pub fn login_response(&self) -> AuthCodes {
        AuthCodes::NotImplemented
    }

    pub fn register_user_response(&self) -> AuthCodes {
        AuthCodes::NotImplemented
    }
}
