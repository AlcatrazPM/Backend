//! # Accounts
//!
//! The `accounts` create deals with all requests to the DataBase.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Trait all controllers need to implement
pub mod accounts_provider;
/// User Account List REST Controller
pub mod controller;
