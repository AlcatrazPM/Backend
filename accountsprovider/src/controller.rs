use rocket::http::Status;
use jwt::apikey::ApiKey;
use rocket_contrib::json::Json;
use userdata::userdata::{AccountsList, AcctCodes, SiteAccount};

#[get("/accounts")]
pub fn get_accounts(key: ApiKey) -> Result<Json<AccountsList>, Status> {
    Err(handle_code(AcctCodes::NotImplemented))
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
    }
}