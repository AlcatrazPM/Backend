use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
// use bson::doc;

use crate::userdata::UserCredentials;

pub fn check_user(_user: UserCredentials) -> bool {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize client.");

    let coll = client.db("alcatraz").collection("users");

    // BIG TODO: Make this work

    // let email = String::from("blankaddress92@gmail.com");
    // let filter = doc! { "email" => email };
    // let filter = doc! { "email" => "blankaddress92@gmail.com" };
    // let filter = bson::ordered::OrderedDocument::new();

    // let result = coll.find_one(Some(filter), None);
    //
    // let vlad = match result {
    //     Ok(maybe_doc) => {
    //         match maybe_doc {
    //             Some(doc) => doc,
    //             None => {
    //                 println!("~No document found");
    //                 return false;
    //             }
    //         }
    //     },
    //     Err(e) => {
    //         println!("{}", e);
    //         return false;
    //     }
    // };


    let cursor = coll.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            println!("email: {:?}", item.get("email"));
        }
    }

    return false;
}