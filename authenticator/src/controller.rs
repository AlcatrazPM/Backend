//! # Authentication Rest Controller
//!
//! Used to execute any all actions needed for authentication

use crate::authenticator::{
    AuthCodes, Authenticator, AuthenticatorControl, Token, UserCredentials,
};
// use serde_json::Value;
use serde::{Deserialize};


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
}

impl<A> AuthenticatorControl for AuthRestController<A>
where
    A: Authenticator,
{
    fn login_response(&self, json: Option<&str>) -> String {
        let mut response: String = format!("HTTP/1.1 400 Bad Request\r\n\r\n");
        if json.is_none() {
            return response;
        }
        let json = json.unwrap();
        let credentials = serde_json::from_str(json);
        if credentials.is_err() {
            return response;
        }
        let credentials = credentials.unwrap();
        println!("~{:?}", credentials);
        let (token, ret_code): (Token, AuthCodes) = self.auth.login(credentials);

        let mut response_json = String::from("{ \"jwt\": \"");
        response_json.push_str(token.as_str());
        response_json.push_str("\" }");

        if ret_code == AuthCodes::LoginOk {
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response_json.len(),
                response_json
            );
        } else if ret_code == AuthCodes::DatabaseError {
            response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        } else if ret_code == AuthCodes::InternalError {
            response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        } else if ret_code == AuthCodes::BadPassword {
            response = format!("HTTP/1.1 401 Unauthorized\r\n\r\n");
        } else if ret_code == AuthCodes::UnregisteredUser {
            response = format!("HTTP/1.1 499 Unregistered User\r\n\r\n");
        } else if ret_code == AuthCodes::NotImplemented {
            response = format!("HTTP/1.1 501 Not Implemented\r\n\r\n");
        }

        response
    }

    fn register_user_response(&self, json: Option<&str>) -> String {
        let mut response: String = format!("HTTP/1.1 400 Bad Request\r\n\r\n");
        if json.is_none() {
            return response;
        }
        let json = json.unwrap();
        let credentials = serde_json::from_str(json);
        if credentials.is_err() {
            return response;
        }
        let credentials = credentials.unwrap();
        println!("~{:?}", credentials);
        let ret_code: AuthCodes = self.auth.register(credentials);

        if ret_code == AuthCodes::RegisterOk {
            response = format!("HTTP/1.1 200 OK\r\n\r\n");
        } else if ret_code == AuthCodes::DatabaseError {
            response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        } else if ret_code == AuthCodes::InternalError {
            response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        } else if ret_code == AuthCodes::NotImplemented {
            response = format!("HTTP/1.1 501 Not Implemented\r\n\r\n");
        }

        response
    }

    fn modify_pass_response(&self, json: Option<&str>) -> String {
        let mut response: String = format!("HTTP/1.1 400 Bad Request\r\n\r\n");
        if json.is_none() {
            return response;
        }
        let json = json.unwrap();
        let json_data = serde_json::from_str(json);
        if json_data.is_err() {
            return response;
        }
        let json_data: ModifyPasswordJSON = json_data.unwrap();
        println!("~~{:?}", json_data);

        let credentials: UserCredentials = UserCredentials {
            username: json_data.username,
            password: json_data.old_password,
        };

        let ret_code: AuthCodes = self
            .auth
            .modify_password(credentials, json_data.new_password);


        if ret_code == AuthCodes::ChangedPassword {
            response = format!("HTTP/1.1 200 OK\r\n\r\n");
        } else if ret_code == AuthCodes::DatabaseError {
            response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        } else if ret_code == AuthCodes::InternalError {
            response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
        } else if ret_code == AuthCodes::BadPassword {
            response = format!("HTTP/1.1 403 Forbidden\r\n\r\n");
        } else if ret_code == AuthCodes::NotImplemented {
            response = format!("HTTP/1.1 501 Not Implemented\r\n\r\n");
        }

        response
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ModifyPasswordJSON {
    username: String,
    old_password: String,
    new_password: String,
}
