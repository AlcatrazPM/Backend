use serde::{Deserialize, Serialize};

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

pub enum UserId {
    ObjectId(bson::oid::ObjectId),
    Email(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseAccountEntry {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub userid: bson::oid::ObjectId,
    pub entries: Vec<DatabaseSiteAccount>,
    pub clear_entries: Vec<DatabaseSiteAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseSiteAccount {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub site: String,
    pub username: String,
    pub password: String,
    pub favorite: bool,
}
