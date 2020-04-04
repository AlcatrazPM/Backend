#[derive(Debug, PartialEq)]
pub enum Requests {
    NotImplemented,
    BadRequest,
    Register,
    Authentication,
    ModifyMasterPassword,
    AccountsList,
    ModifyAccount,
}

pub fn get_request_type(page: &str) -> Requests {
    if page.eq("/register") {
        return Requests::Register;
    } else if page.eq("/authenticate") {
        return Requests::Authentication;
    } else if page.eq("/modifypassword") {
        return Requests::ModifyMasterPassword;
    } else if page.eq("/getaccounts") {
        return Requests::AccountsList;
    } else if page.eq("/modifyaccount") {
        return Requests::ModifyAccount;
    }

    return Requests::BadRequest;
}
