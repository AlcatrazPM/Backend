use crate::dataprovider::primary_data_provider::{change_password, get_user, insert_user};
use crate::jwt::{generate_jwt, JWT};
use crate::userdata::{AuthCodes, ChangePassword, UserCredentials};

#[allow(dead_code)]
pub fn register(user: UserCredentials) -> AuthCodes {
    match insert_user(user) {
        Ok(true) => AuthCodes::RegisterOk,
        Ok(false) => AuthCodes::AlreadyRegistered,
        Err(_) => AuthCodes::DatabaseError,
    }
}

pub fn login(user: UserCredentials) -> JWT {
    match get_user(&user.email) {
        Ok(Some(db_user)) => {
            if db_user.credential == user.password {
                return JWT::JWT(generate_jwt(db_user));
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
        password: data.old_password,
    };
    match change_password(user, data.new_password) {
        Ok(code) => code,
        Err(e) => {
            println!("Error at modified pass: {:?}", e);
            AuthCodes::DatabaseError
        }
    }
}
