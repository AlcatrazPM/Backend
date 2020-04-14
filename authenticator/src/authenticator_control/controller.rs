use crate::authenticator_control::authenticator;
use crate::jwt::{ApiKey, ResponseJWT, JWT};
use crate::userdata::{AuthCodes, ChangePassword, UserCredentials, LoginCredentials};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket_contrib::json::Json;

#[post("/register", data = "<credentials>")]
pub fn register(credentials: Json<UserCredentials>) -> Status {
    println!("{}", format!("Your register data is: {:?}", credentials));
    handle_code(authenticator::register(credentials.0))
}

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<LoginCredentials>) -> Result<Json<ResponseJWT>, Status> {
    // println!("{}", format!("Your login data is: {:?}", credentials));
    match authenticator::login(credentials.0) {
        JWT::JWT(jwt) => Ok(Json(ResponseJWT { jwt })),
        JWT::Error(code) => Err(handle_code(code)),
    }
}

#[post("/modifypassword", data = "<credentials>")]
pub fn modify_password(credentials: Json<ChangePassword>, key: ApiKey) -> Status {
    println!("{}", format!("Your modified data is: {:?}", credentials));
    println!("{}", format!("Your api key is: {:?}", key));
    handle_code(authenticator::modify_password(credentials.0))
}

#[catch(499)]
pub(crate) fn unregistered_user<'r>(req: &'_ Request) -> Response<'r> {
    "Unregistered User".respond_to(req).ok().unwrap()
}

#[catch(498)]
pub(crate) fn already_registered<'r>(req: &'_ Request) -> Response<'r> {
    "Already Registered User".respond_to(req).ok().unwrap()
}

fn handle_code(code: AuthCodes) -> Status {
    match code {
        AuthCodes::UnregisteredUser => Status::new(499, "Unregistered User"),
        AuthCodes::NotImplemented => Status::NotImplemented,
        AuthCodes::DatabaseError => Status::InternalServerError,
        AuthCodes::InternalError => Status::InternalServerError,
        AuthCodes::BadPassword => Status::Unauthorized,
        AuthCodes::ChangedPassword => Status::Ok,
        AuthCodes::RegisterOk => Status::Ok,
        AuthCodes::LoginOk => Status::Ok,
        AuthCodes::AlreadyRegistered => Status::new(498, "Already Registered User"),
    }
}
