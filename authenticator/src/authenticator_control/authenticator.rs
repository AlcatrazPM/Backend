use crate::dataprovider::primary_data_provider::{
    change_account_data, change_password, get_user, insert_user, UserId,
};
use jwt::jwt::{generate_jwt, JWT};
use jwt::claim::Claim;
use std::str::FromStr;
use userdata::userdata::{
    AuthCodes, ChangeAcctData, ChangePassword, LoginCredentials, ParsedChangeAcctData,
    UserCredentials,
};

#[allow(dead_code)]
pub fn register(user: UserCredentials) -> AuthCodes {
    match insert_user(user) {
        Ok(true) => AuthCodes::RegisterOk,
        Ok(false) => AuthCodes::AlreadyRegistered,
        Err(_) => AuthCodes::DatabaseError,
    }
}

pub fn login(user: LoginCredentials) -> JWT {
    match get_user(UserId::Email(user.email)) {
        Ok(Some(db_user)) => {
            if db_user.credential == user.password {
                return JWT::JWT(match generate_jwt(db_user) {
                    Some(jwt) => jwt,
                    None => return JWT::Error(AuthCodes::InternalError),
                });
            }
            JWT::Error(AuthCodes::BadPassword)
        }
        Ok(None) => JWT::Error(AuthCodes::UnregisteredUser),
        Err(_) => JWT::Error(AuthCodes::DatabaseError),
    }
}

#[allow(dead_code)]
pub fn modify_password(data: ChangePassword) -> AuthCodes {
    // unimplemented!()
    // AuthCodes::ChangedPassword
    let user: UserCredentials = UserCredentials {
        email: data.user,
        name: "".to_string(),
        password: data.old_password,
        e_dek: "".to_string(),
        i_kek: "".to_string(),
    };
    match change_password(user, data.new_password) {
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
