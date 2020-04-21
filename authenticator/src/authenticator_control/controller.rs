use crate::authenticator_control::authenticator;
use jwt::apikey::ApiKey;
use jwt::claim::get_claim;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket_contrib::json::Json;
use userdata::userdata::Login;
use userdata::userdata::{
    AuthCodes, ChangeAcctData, ChangePassword, LoginCredentials, LoginResponse, UserCredentials,
};

#[post("/register", data = "<credentials>")]
pub fn register(credentials: Json<UserCredentials>) -> Status {
    println!("{}", format!("Your register data is: {:?}", credentials));
    handle_code(authenticator::register(credentials.0))
}

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<LoginCredentials>) -> Result<Json<LoginResponse>, Status> {
    println!("{}", format!("Your login data is: {:?}", credentials));
    match authenticator::login(credentials.0) {
        Login::Login(response) => Ok(Json(response)),
        Login::Error(code) => Err(handle_code(code)),
    }
}

#[post("/modifypassword", data = "<credentials>")]
pub fn modify_password(credentials: Json<ChangePassword>, key: ApiKey) -> Status {
    println!(
        "{}",
        format!("Your modified password data is: {:?}", credentials)
    );
    println!("{}", format!("Your api key is: {:?}", key));
    handle_code(authenticator::modify_password(credentials.0))
}

#[post("/modifyacctdata", data = "<data>")]
pub fn modify_account_data(data: Json<ChangeAcctData>, key: ApiKey) -> Status {
    println!("{}", format!("Your modified password data is: {:?}", data));
    let claim = match get_claim(key.key.as_str()) {
        Some(data) => data,
        None => return Status::InternalServerError,
    };

    handle_code(authenticator::modify_acct_data(data.0, claim))
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
        AuthCodes::ChangedData => Status::Ok,
        AuthCodes::RegisterOk => Status::Ok,
        AuthCodes::LoginOk => Status::Ok,
        AuthCodes::AlreadyRegistered => Status::new(498, "Already Registered User"),
    }
}
