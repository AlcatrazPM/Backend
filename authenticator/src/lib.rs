//! #Authenticator
//!
//! This crate has the sole purpose of authenticating the user in the system

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Traits all authentication objects need to use
pub mod authenticator;
/// Authentication REST Controller
pub mod controller;
/// Implementation of `Authenticator` trait
pub mod gcp_auth;
