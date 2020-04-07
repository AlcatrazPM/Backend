//! # Authentication Rest Controller
//!
//! Used to execute any all actions needed for authentication

use crate::authenticator::{
    AuthCodes, Authenticator, AuthenticatorControl, Token, UserCredentials,
};
// use serde_json::Value;
use http::response::Response;
use http::statuscode::StatsCodes;
use serde::Deserialize;

/// Authentication Controller
pub struct AuthRestController<A: Authenticator> {
    auth: A,
}

impl<A> AuthRestController<A>
    where
        A: Authenticator,
{
    pub fn new(auth: A) -> AuthRestController<A> {
        AuthRestController { auth }
    }

    fn error(&self, code: AuthCodes) -> Response {
        match code {
            AuthCodes::DatabaseError => Response::build().status(StatsCodes::InternalError),
            AuthCodes::InternalError => Response::build().status(StatsCodes::InternalError),
            AuthCodes::BadPassword => Response::build().status(StatsCodes::Unauthorized),
            AuthCodes::UnregisteredUser => Response::build().status(StatsCodes::UnregisteredUser),
            AuthCodes::NotImplemented => Response::build().status(StatsCodes::NotImplemented),
            _ => Response::build(),
        }
    }
}

impl<A> AuthenticatorControl for AuthRestController<A>
where
    A: Authenticator,
{
    fn login_response(&self, json: Option<&str>) -> Response {
        let response = Response::build().status(StatsCodes::BadRequest);
        let json = match json {
            Some(obj) => obj,
            None => return response,
        };
        let credentials: UserCredentials = match serde_json::from_str(json) {
            Ok(data) => data,
            Err(_) => return response,
        };
        println!("~{:?}", credentials);
        let (token, ret_code): (Token, AuthCodes) = self.auth.login(credentials);

        let mut response_json = String::from("{ \"jwt\": \"");
        response_json.push_str(token.as_str());
        response_json.push_str("\" }");

        match ret_code {
            AuthCodes::LoginOk => {
                let ctype = "application/json".to_string();
                let clength = response_json.len();
                response
                    .status(StatsCodes::OK)
                    .content(ctype, clength as u32)
                    .body(response_json)
            }
            _ => self.error(ret_code),
        }
    }

    fn register_user_response(&self, json: Option<&str>) -> Response {
        let response = Response::build().status(StatsCodes::BadRequest);
        let json = match json {
            Some(obj) => obj,
            None => return response,
        };
        let credentials: UserCredentials = match serde_json::from_str(json) {
            Ok(data) => data,
            Err(_) => return response,
        };
        println!("~{:?}", credentials);
        let ret_code: AuthCodes = self.auth.register(credentials);

        match ret_code {
            AuthCodes::RegisterOk => response.status(StatsCodes::OK),
            _ => self.error(ret_code),
        }
    }

    fn modify_pass_response(&self, json: Option<&str>) -> Response {
        let response = Response::build().status(StatsCodes::BadRequest);

        let json = match json {
            Some(obj) => obj,
            None => return response,
        };

        let json_data: ModifyPasswordJSON = match serde_json::from_str(json) {
            Ok(data) => data,
            Err(_) => return response,
        };
        println!("~~{:?}", json_data);

        let credentials: UserCredentials = UserCredentials {
            username: json_data.username,
            password: json_data.old_password,
        };

        let ret_code: AuthCodes = self
            .auth
            .modify_password(credentials, json_data.new_password);

        match ret_code {
            AuthCodes::ChangedPassword => response.status(StatsCodes::OK),
            _ => self.error(ret_code),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ModifyPasswordJSON {
    username: String,
    old_password: String,
    new_password: String,
}
