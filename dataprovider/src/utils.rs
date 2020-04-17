use mongodb::db::ThreadedDatabase;
use mongodb::{Client, Error, ThreadedClient};

use crate::data_structs::DatabaseUser;
use bson::ordered::OrderedDocument;
use chrono::Utc;
use mongodb::coll::Collection;
use std::env;
use std::str::FromStr;
use userdata::userdata::UserCredentials;

static AUTH_DB: &str = "localhost:27017";

pub(crate) fn parse_db_env_var(env: &str) -> (String, u16) {
    let db = match env::var(env) {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default listening connection for MongoDB");
            AUTH_DB.to_string()
        }
    };

    let db: Vec<&str> = db.split(":").collect();
    let host = match db.get(0) {
        Some(val) => *val,
        None => {
            println!("Setting to default listening connection for MongoDB");
            return ("localhost".to_string(), 27017);
        }
    };
    let port = match db.get(1) {
        Some(val) => (*val),
        None => {
            println!("Setting to default listening connection for MongoDB");
            return ("localhost".to_string(), 27017);
        }
    };
    let port = match u16::from_str(port) {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default listening connection for MongoDB");
            return ("localhost".to_string(), 27017);
        }
    };

    (String::from(host), port)
}

pub(crate) fn connect() -> Result<Collection, Error> {
    let (host, port): (String, u16) = parse_db_env_var("AUTH_DB");
    println!("DB is {}:{}", host, port);

    let client = Client::connect(host.as_str(), port)?;

    Ok(client.db("alcatraz").collection("users"))
}

pub(crate) fn update_user(user: DatabaseUser, filter: OrderedDocument) -> Result<(), Error> {
    let coll = connect()?;

    let serialized_user = bson::to_bson(&user)?;
    if let bson::Bson::Document(document) = serialized_user {
        coll.replace_one(filter, document, None)?;
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        return Err(mongodb::error::Error::DefaultError(
            "converting the BSON object".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn build_db_user(user: UserCredentials) -> Result<DatabaseUser, Error> {
    Ok(DatabaseUser {
        id: bson::oid::ObjectId::new()?,
        email: user.email,
        name: user.name,
        session_timer: 15, // default timer
        credential: user.password,
        date: Utc::now().to_string(),
        // e_dek: "SuchSecurity".to_string(),
        // i_kek: "MuchEncryption".to_string(),
        e_dek: user.e_dek,
        i_kek: user.i_kek,
    })
}
