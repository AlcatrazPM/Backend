use dataprovider::data_structs::UserId;
use dataprovider::primary_data_provider::{
    change_account_data, change_password, get_user, insert_user,
};
use jwt::claim::Claim;
use jwt::jwt::generate_jwt;
use std::str::FromStr;
use userdata::userdata::{
    AuthCodes, ChangeAcctData, ChangePassword, Login, LoginCredentials, LoginResponse,
    ParsedChangeAcctData, UserCredentials,
};

#[allow(dead_code)]
pub fn register(user: UserCredentials) -> AuthCodes {
    match insert_user(user) {
        Ok(true) => AuthCodes::RegisterOk,
        Ok(false) => AuthCodes::AlreadyRegistered,
        Err(_) => AuthCodes::DatabaseError,
    }
}

pub fn login(user: LoginCredentials) -> Login {
    let db_user = match get_user(UserId::Email(user.email)) {
        Ok(Some(usr)) if usr.credential == user.password => usr,
        Ok(Some(_)) => return Login::Error(AuthCodes::BadPassword),
        Ok(None) => return Login::Error(AuthCodes::UnregisteredUser),
        Err(_) => return Login::Error(AuthCodes::DatabaseError),
    };

    let jwt = match generate_jwt(&db_user) {
        Some(jwt) => jwt,
        None => return Login::Error(AuthCodes::InternalError),
    };

    Login::Login(LoginResponse {
        name: db_user.name,
        session_timer: db_user.session_timer,
        e_dek: db_user.e_dek,
        i_kek: db_user.i_kek,
        jwt,
    })
}

#[allow(dead_code)]
pub fn modify_password(data: ChangePassword, claim: Claim) -> AuthCodes {
    // unimplemented!()
    // AuthCodes::ChangedPassword
    let db_user = match get_user(UserId::Email(data.user.clone())) {
        Ok(Some(user)) if user.credential == data.old_password && user.id.to_hex() == claim.usr => {
            user
        }
        Ok(Some(_)) => return AuthCodes::BadPassword,
        Ok(None) => return AuthCodes::UnregisteredUser,
        Err(_) => return AuthCodes::DatabaseError,
    };

    match change_password(db_user, data) {
        Ok(code) => code,
        Err(e) => {
            println!("Error at modified pass: {:?}", e);
            AuthCodes::DatabaseError
        }
    }
}

pub fn modify_acct_data(data: ChangeAcctData, claim: Claim) -> AuthCodes {
    let id = match bson::oid::ObjectId::with_string(claim.usr.as_str()) {
        Ok(id) => id,
        Err(_) => return AuthCodes::InternalError,
    };

    println!("change (id: {:?} ) is: {:?}", id, data);

    let parsed_data = match data.field_name.as_str() {
        "email" => ParsedChangeAcctData::Email(data.new_value),
        "name" => ParsedChangeAcctData::Name(data.new_value),
        "session_timer" => {
            let timer = match i64::from_str(data.new_value.as_str()) {
                Ok(t) => t,
                Err(_) => return AuthCodes::InternalError,
            };
            ParsedChangeAcctData::SessionTimer(timer)
        }
        _ => return AuthCodes::InternalError,
    };

    println!("parsed data is: {:?}", parsed_data);

    match change_account_data(id, parsed_data) {
        Ok(code) => code,
        Err(e) => {
            println!("Error at modified other data: {:?}", e);
            AuthCodes::DatabaseError
        }
    }
}
