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
    match page {
        "/register" => Requests::Register,
        "/login" => Requests::Authentication,
        "/modifypassword" => Requests::ModifyMasterPassword,
        "/getaccounts" => Requests::AccountsList,
        "/modifyaccount" => Requests::ModifyAccount,
        _ => Requests::BadRequest,
    }
}

pub fn authorise(request: &String) -> Option<&str> {
    let auth_field = match request.find("Authorization") {
        Some(field) => field,
        None => return None,
    };

    let jwt: &str = &request[auth_field..]
        .split_terminator("\r\n")
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()[..];

    // println!("~{}~", jwt);
    // TODO: Add jwt crate and actually check jwt
    if is_valid(jwt) {
        return Some(jwt);
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
