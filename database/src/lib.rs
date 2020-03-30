//! # DataBase Connection Crate
//!
//! It accomplishes the TCP connection with the DB.
//! Purpose of issuing commands and retrieving data.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
/// Interface to build a Data Provider
pub mod data_provider;
/// Main method of interfacing the DataBase
pub mod primary_data_provider;
