use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserCredentials {
    #[serde(alias = "Username")]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangePassword {
    #[serde(alias = "Username")]
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
    pub(crate) id: bson::oid::ObjectId,
    pub(crate) clear_entries: Vec<SiteAccount>,
    pub(crate) credential: String,
    pub(crate) date: String,
    pub(crate) e_dek: String,
    pub email: String,
    pub(crate) i_kek: String,
    pub(crate) secure_entries: Vec<SiteAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteAccount {
    pub(crate) pass: String,
    pub(crate) site: String,
    pub(crate) user: String,
}
