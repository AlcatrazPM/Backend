//! # Site Info
//!
//! Pretty self explanatory

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SiteAccount {
    pub username: String,
    pub password: String,
    pub id: String,
}
