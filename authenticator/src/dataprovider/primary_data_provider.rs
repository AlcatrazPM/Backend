use mongodb::db::ThreadedDatabase;
use mongodb::{Client, Error, ThreadedClient};

use crate::userdata::{AuthCodes, DatabaseUser, UserCredentials};
use chrono::Utc;
use mongodb::coll::Collection;

fn connect() -> Result<Collection, Error> {
    let client = Client::connect("localhost", 27017)?;

    Ok(client.db("alcatraz").collection("users"))
}

pub fn get_user(_email: &str) -> Result<Option<DatabaseUser>, Error> {
    let coll = connect()?;

    let filter = doc! { "email" => _email };
    let result = coll.find_one(Some(filter), None);

    let user = match result {
        Ok(maybe_doc) => match maybe_doc {
            Some(doc) => doc,
            None => {
                println!("~No document found");
                return Ok(None);
            }
        },
        Err(e) => return Err(e),
    };
    let user: DatabaseUser = bson::from_bson(bson::Bson::Document(user))?;
    println!("~Database found user: {:?}", user.email);

    Ok(Some(user))
}

pub fn insert_user(user: UserCredentials) -> Result<bool, Error> {
    let coll = connect()?;
    if let Ok(Some(_)) = get_user(&user.email) {
        return Ok(false);
    }

    let user: DatabaseUser = build_db_user(user)?;
    println!("insert this: {:?}", user);
    let serialized_user = bson::to_bson(&user)?;
    if let bson::Bson::Document(document) = serialized_user {
        coll.insert_one(document, None)?;
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        return Err(mongodb::error::Error::DefaultError(
            "converting the BSON object".to_string(),
        ));
    }
    Ok(true)
}

pub fn change_password(user: UserCredentials, new_password: String) -> Result<AuthCodes, Error> {
    let coll = connect()?;
    let mut user = match get_user(&user.email) {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(AuthCodes::UnregisteredUser),
        Err(e) => return Err(e),
    };
    let filter = doc! { "email" => user.email.clone() };

    user.credential = new_password;
    // println!("update this: {:?}", &user);
    let serialized_user = bson::to_bson(&user)?;
    if let bson::Bson::Document(document) = serialized_user {
        coll.replace_one(filter, document, None)?;
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        return Err(mongodb::error::Error::DefaultError(
            "converting the BSON object".to_string(),
        ));
    }

    Ok(AuthCodes::ChangedPassword)
}

fn build_db_user(user: UserCredentials) -> Result<DatabaseUser, Error> {
    Ok(DatabaseUser {
        id: bson::oid::ObjectId::new()?,
        email: user.email,
        name: "Placeholder Dorel".to_string(),
        session_timer: 15,      // default timer
        credential: user.password,
        date: Utc::now().to_string(),
        // e_dek: "SuchSecurity".to_string(),
        // i_kek: "MuchEncryption".to_string(),
        e_dek: user.e_dek,
        i_kek: user.i_kek,
    })
}
