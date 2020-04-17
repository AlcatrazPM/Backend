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