use mongodb::Error;

use userdata::userdata::{AuthCodes, DatabaseUser, ParsedChangeAcctData, UserCredentials};
use crate::utils;
use crate::data_structs::UserId;


pub fn get_user(id: UserId) -> Result<Option<DatabaseUser>, Error> {
    let coll = utils::connect()?;

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
    let coll = utils::connect()?;
    if let Ok(Some(_)) = get_user(UserId::Email(user.email.clone())) {
        return Ok(false);
    }

    let user: DatabaseUser = utils::build_db_user(user)?;
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
    let mut user = match get_user(UserId::Email(user.email)) {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(AuthCodes::UnregisteredUser),
        Err(e) => return Err(e),
    };
    let filter = doc! { "email" => user.email.clone() };

    user.credential = new_password;

    match utils::update_user(user, filter) {
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

    match utils::update_user(user, filter) {
        Ok(_) => Ok(AuthCodes::ChangedData),
        Err(e) => Err(e),
    }
}



