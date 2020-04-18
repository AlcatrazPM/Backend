use chrono::{Duration, Utc};

extern crate jsonwebtoken;

use self::jsonwebtoken::{encode, Header};
use crate::claim::{get_claim, Claim};
use dataprovider::data_structs::DatabaseUser;
use std::env;

pub static JWT_KEY: &str = "very_secret";

pub fn generate_jwt(user: &DatabaseUser) -> Option<String> {
    let secret = match env::var("KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default secret for jwt");
            JWT_KEY.to_string()
        }
    };
    println!("secret is {}", secret);

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
