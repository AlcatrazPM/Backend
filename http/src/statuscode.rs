// for any new codes, please add in order
#[derive(Debug)]
pub enum StatsCodes {
    OK,                 // 200
    BadRequest,         // 400
    Unauthorized,       // 401
    Forbidden,          // 403
    NotFound,           // 404
    UnregisteredUser,   // 499
    InternalError,      // 500
    NotImplemented,     // 501
}



impl StatsCodes {

    pub fn get_code_no(&self) -> (u16, String) {
        let (code, name): (u16, String) = match self {
            StatsCodes::OK=> (200, String::from("OK")),
            StatsCodes::BadRequest => (400, String::from("Bad Request")),
            StatsCodes::Unauthorized => (401, String::from("Unauthorized")),
            StatsCodes::Forbidden => (403, String::from("Forbidden")),
            StatsCodes::NotFound => (404, String::from("Not Found")),
            StatsCodes::UnregisteredUser => (499, String::from("Unregistered User")),
            StatsCodes::InternalError => (500, String::from("Internal Error")),
            StatsCodes::NotImplemented => (501, String::from("Not Implemented")),
        };

        (code, name)
    }
    pub fn is_success(&self) -> bool {
        match self {
            StatsCodes::OK => true,
            _ => false,
        }
    }

    pub fn is_client_error(&self) -> bool {
        match self {
            StatsCodes::BadRequest => true,
            StatsCodes::Unauthorized => true,
            StatsCodes::Forbidden => true,
            StatsCodes::NotFound => true,
            StatsCodes::UnregisteredUser => true,
            _ => false,
        }
    }

    pub fn is_server_error(&self) -> bool {
        match self {
            StatsCodes::InternalError => true,
            StatsCodes::NotImplemented => true,
            _ => false,
        }
    }
}

impl ToString for StatsCodes {
    fn to_string(&self) -> String {
        let (code, name) = self.get_code_no();
        format!("{} {}", code, name)
    }
}