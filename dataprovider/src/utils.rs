use mongodb::db::ThreadedDatabase;
use mongodb::{Client, Error, ThreadedClient};

use crate::data_structs::{DatabaseAccountEntry, DatabaseUser};
use bson::ordered::OrderedDocument;
use chrono::Utc;
use mongodb::coll::Collection;
use std::env;
use std::str::FromStr;
use userdata::userdata::UserCredentials;

static AUTH_DB: &str = "localhost:27017";
static ACCT_DB: &str = "localhost:27018";

pub(crate) fn parse_db_env_var(env: &str) -> (String, u16, String, String) {
    let db = match env::var(env) {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default listening connection for MongoDB");
            match env {
                "AUTH_DB" => AUTH_DB.to_string(),
                "ACCT_DB" => ACCT_DB.to_string(),
                _ => AUTH_DB.to_string(),
            }
            // AUTH_DB.to_string()
        }
    };

    let db: Vec<&str> = db.split(':').collect();
    let host = match db.get(0) {
        Some(val) => *val,
        None => {
            println!("Setting to default listening connection for MongoDB");
            return ("localhost".to_string(), 27017, "dorel".to_string(), "doreldorel".to_string());
        }
    };
    let port = match db.get(1) {
        Some(val) => (*val),
        None => {
            println!("Setting to default listening connection for MongoDB");
            return ("localhost".to_string(), 27017, "dorel".to_string(), "doreldorel".to_string());

        }
    };
    let port = match u16::from_str(port) {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default listening connection for MongoDB");
            return ("localhost".to_string(), 27017, "dorel".to_string(), "doreldorel".to_string());

        }
    };
    let user = match env::var("MONGO_USER") {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default listening connection for MongoDB");
            "dorel".to_string()
        }
    };
    let pass = match env::var("MONGO_PASS") {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default listening connection for MongoDB");
            "doreldorel".to_string()
        }
    };

    (String::from(host), port, user, pass)
}

pub(crate) fn connect(db: DB) -> Result<Collection, Error> {
    let db_type: &str = match db {
        DB::Auth => "AUTH_DB",
        DB::Acct => "ACCT_DB",
    };
    let (host, port, user, pass): (String, u16, String, String) = parse_db_env_var(db_type);

    let client = Client::connect(host.as_str(), port)?;
    let dbb = client.db("admin");
    let auth = dbb.auth(&user, &pass);

    let coll_name: &str = match db {
        DB::Auth => "users",
        DB::Acct => "entries",
    };

    Ok(client.db("alcatraz").collection(coll_name))
}

pub(crate) fn update_user(user: DatabaseUser, filter: OrderedDocument) -> Result<(), Error> {
    let coll = connect(DB::Auth)?;

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

pub(crate) fn build_db_acct(id: bson::oid::ObjectId) -> Result<DatabaseAccountEntry, Error> {
    Ok(DatabaseAccountEntry {
        id: bson::oid::ObjectId::new()?,
        userid: id,
        entries: vec![],
        clear_entries: vec![],
    })
}

pub(crate) fn update_acct_user(
    user: DatabaseAccountEntry,
    filter: OrderedDocument,
) -> Result<(), Error> {
    let coll = connect(DB::Acct)?;

    let serialized_user = bson::to_bson(&user)?;
    if let bson::Bson::Document(document) = serialized_user {
        print!("");
        coll.replace_one(filter, document, None)?;
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        return Err(mongodb::error::Error::DefaultError(
            "converting the BSON object".to_string(),
        ));
    }
    Ok(())
}

pub(crate) enum DB {
    Auth,
    Acct,
}