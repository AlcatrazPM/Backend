use mongodb::db::ThreadedDatabase;
use mongodb::{Client, Error, ThreadedClient};

use bson::ordered::OrderedDocument;
use chrono::Utc;
use mongodb::coll::Collection;
use std::env;
use std::str::FromStr;
use userdata::userdata::{AuthCodes, DatabaseUser, ParsedChangeAcctData, UserCredentials};

static AUTH_DB: &str = "localhost:27017";

fn parse_db_env_var(env: &str) -> (String, u16) {
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

fn connect() -> Result<Collection, Error> {
    let (host, port): (String, u16) = parse_db_env_var("AUTH_DB");
    println!("DB is {}:{}", host, port);

    let client = Client::connect(host.as_str(), port)?;

    Ok(client.db("alcatraz").collection("users"))
}

pub fn get_user(id: UserId) -> Result<Option<DatabaseUser>, Error> {
    let coll = connect()?;

    let filter = match id {
        UserId::ObjectId(oid) => doc! { "_id": oid },
        UserId::Email(email) => doc! { "email" => email },
    };

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
    if let Ok(Some(_)) = get_user(UserId::Email(user.email.clone())) {
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

fn update_user(user: DatabaseUser, filter: OrderedDocument) -> Result<(), Error> {
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

pub fn change_password(user: UserCredentials, new_password: String) -> Result<AuthCodes, Error> {
    let mut user = match get_user(UserId::Email(user.email)) {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(AuthCodes::UnregisteredUser),
        Err(e) => return Err(e),
    };
    let filter = doc! { "email" => user.email.clone() };

    user.credential = new_password;

    match update_user(user, filter) {
        Ok(_) => Ok(AuthCodes::ChangedPassword),
        Err(e) => Err(e),
    }
}

pub fn change_account_data(
    id: bson::oid::ObjectId,
    data: ParsedChangeAcctData,
) -> Result<AuthCodes, Error> {
    let mut user = match get_user(UserId::ObjectId(id.clone())) {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(AuthCodes::UnregisteredUser),
        Err(e) => return Err(e),
    };

    println!("user is: {:?}", user);

    match data {
        ParsedChangeAcctData::Email(email) => user.email = email,
        ParsedChangeAcctData::Name(name) => user.name = name,
        ParsedChangeAcctData::SessionTimer(timer) => user.session_timer = timer,
    };

    println!("user is: {:?}", user);

    let filter = doc! { "_id": id };

    match update_user(user, filter) {
        Ok(_) => Ok(AuthCodes::ChangedData),
        Err(e) => Err(e),
    }
}

fn build_db_user(user: UserCredentials) -> Result<DatabaseUser, Error> {
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

pub enum UserId {
    ObjectId(bson::oid::ObjectId),
    Email(String),
}
