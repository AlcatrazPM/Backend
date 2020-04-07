//! # Accounts REST Controller
//!
//! This module implements the `AccountsProvider` trait.

use crate::accounts_provider::{AccountsCodes, AccountsProvider, AccountsControl};
use userdata::siteinfo::SiteAccount;
use serde::{Serialize, Deserialize};
use http::response::Response;
use http::statuscode::StatsCodes;

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
        // let mut response: String = format!("HTTP/1.1 400 Bad Request\r\n\r\n");
        let response = Response::build().status(StatsCodes::BadRequest);

        if json.is_none() {
            return response;
        }
        let json = json.unwrap();
        let action = serde_json::from_str(json);
        if action.is_err() {
            return response;
        }
        let action: AcctAction = action.unwrap();
        // println!("~{:?}", action);

        // TODO: Decode jwt to find user and then do action
        let user_id: String = "ILoveRust".to_string();

        let mut ret_code = AccountsCodes::NotImplemented;
        if action.operation == "add".to_string() {
            ret_code = self.provider.add_site_account(user_id, action.site);
        } else if action.operation == "remove".to_string() {
            ret_code = self.provider.remove_site_account(user_id, action.site);
        } else if action.operation == "modify".to_string() {
            ret_code = self.provider.modify_site_account(user_id, action.site);
        }

        if ret_code != AccountsCodes::OperationOK {
            // TODO: respond to each error
            return response;
        }

        // response = format!("HTTP/1.1 200 OK\r\n\r\n");
        Response::build().status(StatsCodes::OK)
    }

    fn get_all_site_accounts_response(&self, jwt: &str) -> Response {
        println!("you get an account and you get an account, everybody gets an account");
        // let mut response: String = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");
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
        // println!("/{}/", json)

        // response = format!(
        //     "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        //     json.len(),
        //     json
        // );
        Response::build()
            .status(StatsCodes::OK)
            .content("application/json".to_string(), json.len() as u32)
            .body(json)
        //end mock response

        // response
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
