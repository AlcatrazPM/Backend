//! # Accounts REST Controller
//!
//! This module implements the `AccountsProvider` trait.

use crate::accounts_provider::{AccountsCodes, AccountsControl, AccountsProvider};
use http::response::Response;
use http::statuscode::StatsCodes;
use serde::{Deserialize, Serialize};
use userdata::siteinfo::SiteAccount;

pub struct AccountsRestController<A: AccountsProvider> {
    provider: A,
}

impl<A> AccountsRestController<A>
    where
        A: AccountsProvider,
{
    pub fn new(provider: A) -> AccountsRestController<A> {
        AccountsRestController { provider }
    }
}

impl<A> AccountsControl for AccountsRestController<A>
where
    A: AccountsProvider,
{
    fn modify_site_account_response(&self, json: Option<&str>, jwt: &str) -> Response {
        let response = Response::build().status(StatsCodes::BadRequest);

        let json = match json {
            Some(obj) => obj,
            None => return response,
        };

        let action: AcctAction = match serde_json::from_str(json) {
            Ok(obj) => obj,
            Err(_) => return response,
        };
        // println!("~{:?}", action);

        // TODO: Decode jwt to find user and then do action
        let user_id: String = "ILoveRust".to_string();

        let ret_code: AccountsCodes = match action.operation.as_str() {
            "add" => self.provider.add_site_account(user_id, action.site),
            "remove" => self.provider.remove_site_account(user_id, action.site),
            "modify" => self.provider.modify_site_account(user_id, action.site),
            _ => AccountsCodes::NotImplemented,
        };

        match ret_code {
            AccountsCodes::OperationOK => Response::build().status(StatsCodes::OK),
            _ => Response::build().status(StatsCodes::BadRequest),
        }
    }

    fn get_all_site_accounts_response(&self, jwt: &str) -> Response {
        println!("you get an account and you get an account, everybody gets an account");
        let response = Response::build().status(StatsCodes::InternalError);

        // mock response
        // TODO: Decode jwt to find user and then query accounts
        let user_id: String = "ILoveRust".to_string();
        let (accounts, ret_code) = self.provider.get_all_site_accounts(user_id);
        if ret_code != AccountsCodes::OperationOK {
            return response;
        }
        let acctlist = AcctList { accounts };
        let json = serde_json::to_string(&acctlist);
        if json.is_err() {
            return response;
        }
        let json = json.unwrap();

        Response::build()
            .status(StatsCodes::OK)
            .content("application/json".to_string(), json.len() as u32)
            .body(json)
        //end mock response
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AcctList {
    accounts: Vec<SiteAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AcctAction {
    operation: String,
    site: SiteAccount,
}
