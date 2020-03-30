//! # UserData
//!
//! Simple Plain Old Rust Objects used in various crates in the password manager.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Information about sites saved in the manager
pub mod siteinfo;
/// Information about the user of the app
pub mod userinfo;
