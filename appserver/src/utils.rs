
#[allow(dead_code)]
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
    } else if page.eq("/login") {
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

pub fn authorise(request: &String) -> Option<&str> {
    let auth_field = request.find("Authorization");
    if auth_field.is_none() {
        return None;
    }

    let jwt: &str = &request[auth_field.unwrap()..]
        .split_terminator("\r\n")
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()[..];

    // println!("~{}~", jwt);
    // TODO: Add jwt crate and actually check jwt
    if is_valid(jwt) {
        return Some(jwt)
    }

    None
}

fn is_valid(jwt: &str) -> bool {
    if jwt == "abCdeFGhi.JkLmNoPQRS.tuVWXyZ" {
        println!("JWT Valid");
        return true;
    }
    println!("JWT `NOT` Valid");
    false
}