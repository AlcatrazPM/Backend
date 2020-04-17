use chrono::{Duration, Utc};
// use rocket::http::Status;
// use rocket::request::{self, FromRequest, Request};
// use rocket::Outcome;
use serde::{Serialize};
use userdata::userdata::{AuthCodes, DatabaseUser};

extern crate jsonwebtoken;

use self::jsonwebtoken::{encode, Header};
use std::env;
use crate::claim::{Claim, get_claim};

pub static JWT_KEY: &str = "very_secret";

pub enum JWT {
    JWT(String),
    Error(AuthCodes),
}

#[derive(Debug, Serialize)]
pub struct ResponseJWT {
    pub jwt: String,
}

pub fn generate_jwt(user: DatabaseUser) -> Option<String> {
    let secret = match env::var("KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default secret for jwt");
            JWT_KEY.to_string()
        }
    };
    println!("secret is {}", secret);
    // println!("_id is {}, serialize: {} and as deserialize: {}", user.id, user.id.to_hex(), bson::oid::ObjectId);

    let claim = Claim {
        // exp: (Utc::now() + Duration::minutes(1)),
        exp: (Utc::now() + Duration::minutes(user.session_timer as i64)),
        // exp: (Utc::now() + Duration::minutes(3)),
        iat: Utc::now(),
        iss: "AlcatrazAuth".to_string(),
        usr: user.id.to_hex(),
    };

    match encode(&Header::default(), &claim, secret.as_ref()) {
        Ok(jwt) => Some(jwt),
        Err(_) => None,
    }
}

pub fn is_valid(jwt: &str) -> bool {
    match get_claim(jwt) {
        Some(_) => true,
        None => false,
    }
}
