use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct UserCredentials {
    #[serde(alias = "username")]
    pub email: String,
    pub password: String,
    pub e_dek: String,
    pub i_kek: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    #[serde(alias = "username")]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct ChangePassword {
    #[serde(alias = "username")]
    pub user: String,
    pub old_password: String,
    pub new_password: String,
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
    AlreadyRegistered,
    LoginOk,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseUser {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub email: String,
    pub name: String,
    pub date: String,
    pub session_timer: i64,
    pub credential: String,
    pub e_dek: String,
    pub i_kek: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteAccount {
    pub(crate) pass: String,
    pub(crate) site: String,
    pub(crate) user: String,
}
