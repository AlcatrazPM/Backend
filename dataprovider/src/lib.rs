#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use(bson, doc)]
extern crate bson;

pub mod data_structs;
pub mod primary_data_provider;
mod utils;
