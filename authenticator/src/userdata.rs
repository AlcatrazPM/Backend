use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserCredentials {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePassword {
    #[serde(alias = "email")]
    user: String,
    old_password: String,
    new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseJWT {
    pub jwt: String,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum AuthCodes {
    NotImplemented,
    DatabaseError,
    InternalError,
    UnregisteredUser,
    BadPassword,
    ChangedPassword,
    RegisterOk,
    LoginOk,
}
