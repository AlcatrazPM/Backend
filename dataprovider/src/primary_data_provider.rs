use mongodb::Error;

use crate::data_structs::{DatabaseAccountEntry, DatabaseUser, UserId};
use crate::utils;
use crate::utils::{build_db_acct, DB};
use userdata::userdata::{AuthCodes, ChangePassword, ParsedChangeAcctData, UserCredentials};

pub fn get_user(id: UserId) -> Result<Option<DatabaseUser>, Error> {
    let coll = utils::connect(DB::Auth)?;

    let filter = match id {
        UserId::ObjectId(oid) => doc! { "_id": oid },
        UserId::Email(email) => doc! { "email" => email },
    };

    println!("filter is: {:?}", filter);
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
    let coll = utils::connect(DB::Auth)?;
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

pub fn change_password(mut user: DatabaseUser, data: ChangePassword) -> Result<AuthCodes, Error> {
    // let mut user = match get_user(UserId::Email(data.user)) {
    //     Ok(Some(u)) => u,
    //     Ok(None) => return Ok(AuthCodes::UnregisteredUser),
    //     Err(e) => return Err(e),
    // };
    let filter = doc! { "email" => user.email.clone() };

    user.credential = data.new_password;
    user.e_dek = data.new_dek;

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

pub fn get_accounts_list(id: UserId) -> Result<Option<DatabaseAccountEntry>, Error> {
    let coll = utils::connect(DB::Acct)?;

    let filter = match id {
        UserId::ObjectId(oid) => doc! { "userid": oid },
        UserId::Email(_) => return Err(mongodb::Error::ArgumentError("wrong id".to_string())),
    };

    println!("filter is: {:?}", filter);
    let result = coll.find_one(Some(filter), None);

    let entry = match result {
        Ok(maybe_doc) => match maybe_doc {
            Some(doc) => doc,
            None => {
                println!("~No entry found");
                return Ok(None);
            }
        },
        Err(e) => return Err(e),
    };

    println!("document found is: {:?}", entry);
    let entry: DatabaseAccountEntry = bson::from_bson(bson::Bson::Document(entry))?;
    println!("~Database found entry: {:?}", entry.userid);

    Ok(Some(entry))
}

pub fn create_acct_user(id: bson::oid::ObjectId) -> Result<Option<DatabaseAccountEntry>, Error> {
    let coll = utils::connect(DB::Acct)?;
    let entry: DatabaseAccountEntry = build_db_acct(id.clone())?;
    println!("insert this: {:?}", entry);
    let serialized_entry = bson::to_bson(&entry)?;

    if let bson::Bson::Document(document) = serialized_entry {
        print!("");
        coll.insert_one(document, None)?;
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        return Err(mongodb::error::Error::DefaultError(
            "converting the BSON object".to_string(),
        ));
    }

    get_accounts_list(UserId::ObjectId(id))
}

pub fn update_accounts(accounts: DatabaseAccountEntry) -> Result<(), Error> {
    let filter = doc! { "_id": accounts.id.clone() };

    println!("filter is: {:?}", filter);
    println!("database object is: {:?}", accounts);

    utils::update_acct_user(accounts, filter)?;

    Ok(())
}
