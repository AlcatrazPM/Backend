use crate::userdata::{AuthCodes, DatabaseUser};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::Serialize;

pub enum JWT {
    JWT(String),
    Error(AuthCodes),
}

#[derive(Debug, Serialize)]
pub struct ResponseJWT {
    pub jwt: String,
}

pub fn generate_jwt(_user: DatabaseUser) -> String {
    // TODO: Generate JWT
    "abc.def.ghi".to_string()
}

pub fn is_valid(jwt: &str) -> bool {
    println!("Key is {}", jwt);
    if jwt.eq("Bearer abc.def.ghi") {
        return true;
    }
    false
}

#[derive(Debug)]
pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // unimplemented!()
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::Forbidden, ApiKeyError::BadCount)),
        }
    }
}
