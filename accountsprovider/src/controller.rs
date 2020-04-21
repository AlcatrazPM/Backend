use rocket::http::Status;
use jwt::apikey::ApiKey;
use rocket_contrib::json::Json;
use userdata::userdata::{AccountsList, AcctCodes, SiteAccount, Accounts};
use jwt::claim::get_claim;
use crate::accountsprovider;

#[get("/accounts")]
pub fn get_accounts(key: ApiKey) -> Result<Json<AccountsList>, Status> {
    let claim = match get_claim(key.key.as_str()) {
        Some(data) => data,
        None => return Err(Status::InternalServerError),
    };
    match accountsprovider::get_accounts(claim) {
        Accounts::Accounts(list) => Ok(Json(list)),
        Accounts::Error(code) => Err(handle_code(code)),
    }
}

#[put("/modifyaccount", data = "<site>")]
pub fn modify_account(key: ApiKey, site: Json<SiteAccount>) -> Status {
    handle_code(AcctCodes::NotImplemented)
}

#[delete("/modifyaccount", data = "<site>")]
pub fn delete_account(key: ApiKey, site: Json<SiteAccount>) -> Status {
    handle_code(AcctCodes::NotImplemented)
}


fn handle_code(code: AcctCodes) -> Status {
    match code {
        AcctCodes::NotImplemented => Status::NotImplemented,
        AcctCodes::DatabaseError => Status::InternalServerError,
        AcctCodes::InternalError => Status::InternalServerError,
        AcctCodes::ChangedData => Status::Ok,
        AcctCodes::NoSuchUser => Status::ImATeapot,
    }
}