use crate::authenticator_control::authenticator;
use crate::userdata::{AuthCodes, ChangePassword, ResponseJWT, UserCredentials};
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
pub fn login(credentials: Json<UserCredentials>) -> Result<Json<ResponseJWT>, Status> {
    println!("{}", format!("Your login data is: {:?}", credentials));
    if let Some(jwt) = authenticator::login(credentials.0) {
        return Ok(Json(ResponseJWT { jwt }));
    }

    Err(Status::new(499, "Unregistered User"))
}

#[post("/modifypassword", data = "<credentials>")]
pub fn modify_password(credentials: Json<ChangePassword>) -> Status {
    println!("{}", format!("Your modified data is: {:?}", credentials));
    // match authenticator::modify_password(credentials.0) {
    //     AuthCodes::ChangedPassword => Status::Ok,
    //     _ => Status::NotImplemented,
    // }
    handle_code(authenticator::modify_password(credentials.0))
}

#[catch(499)]
pub(crate) fn unregistered_user<'r>(req: &'_ Request) -> Response<'r> {
    "Unregistered User".respond_to(req).ok().unwrap()
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
    }
}
