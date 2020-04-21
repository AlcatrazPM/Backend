use dataprovider::data_structs::UserId;
use dataprovider::primary_data_provider::get_accounts_list;
use jwt::claim::Claim;
use std::env;
use userdata::userdata::{Accounts, AccountsList, AcctCodes, SiteAccount};

#[derive(Debug)]
enum EntryType {
    Clear,
    Encrypted,
}

pub(crate) fn get_accounts(claim: Claim) -> Accounts {
    let oid = match bson::oid::ObjectId::with_string(claim.usr.as_str()) {
        Ok(id) => id,
        Err(e) => {
            println!("Error at converting ObjectId {:?}", e);
            return Accounts::Error(AcctCodes::InternalError);
        }
    };
    println!("ObjectId to search is: {:?}", oid);
    let db_entries = match get_accounts_list(UserId::ObjectId(oid)) {
        Ok(Some(entry)) => entry,
        Ok(None) => return Accounts::Error(AcctCodes::NoSuchUser),
        Err(e) => {
            println!("aError: {:?}", e);
            return Accounts::Error(AcctCodes::DatabaseError);
        }
    };

    println!("db found: {:?}", db_entries);

    // check if need to return in clear or encrypted

    let entry_ret_type = match env::var("CLEAR") {
        Ok(_) => EntryType::Clear,
        Err(_) => EntryType::Encrypted,
    };
    println!("entry type is: {:?}", entry_ret_type);

    // populate Response with entries

    let entries = match entry_ret_type {
        EntryType::Clear => db_entries.clear_entries,
        EntryType::Encrypted => db_entries.entries,
    };

    let mut accounts: Vec<SiteAccount> = Vec::new();

    for entry in entries {
        accounts.push(SiteAccount {
            id: entry.id.to_hex(),
            site: entry.site,
            username: entry.username,
            password: entry.password,
            favorite: entry.favorite,
        })
    }

    println!("Accounts to returns is: {:?}", accounts);

    Accounts::Accounts(AccountsList { accounts })
}
