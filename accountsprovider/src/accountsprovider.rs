use jwt::claim::Claim;
use userdata::userdata::{Accounts, AcctCodes};

pub(crate) fn get_accounts(claim: Claim) -> Accounts {
    Accounts::Error(AcctCodes::NotImplemented)
}