use crate::userdata::{AuthCodes, ChangePassword, UserCredentials};
use crate::dataprovider::primary_data_provider::check_user;

#[allow(dead_code)]
pub fn register(_user: UserCredentials) -> AuthCodes {
    // unimplemented!()
    AuthCodes::RegisterOk
}

#[allow(dead_code)]
pub fn login(user: UserCredentials) -> Option<String> {
    // TODO: Change the String return type to JWT
    // unimplemented!()
    check_user(user);
    Some("abc.def.ghi".to_string())
}

#[allow(dead_code)]
pub fn modify_password(_data: ChangePassword) -> AuthCodes {
    // unimplemented!()
    AuthCodes::ChangedPassword
}
