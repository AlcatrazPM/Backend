use dataprovider::data_structs::{DatabaseAccountEntry, DatabaseSiteAccount, UserId};
use dataprovider::primary_data_provider::{create_acct_user, get_accounts_list, update_accounts};
use jwt::claim::Claim;
use std::env;
use userdata::userdata::{Accounts, AccountsList, AcctCodes, SiteAccount, SiteAccountAction};

#[derive(Debug)]
enum EntryType {
    Clear,
    Encrypted,
}

#[derive(Debug)]
enum DbEntry {
    Entry(DatabaseAccountEntry),
    Error(AcctCodes),
}

fn get_db_entry(claim: &Claim) -> DbEntry {
    let oid = match bson::oid::ObjectId::with_string(claim.usr.as_str()) {
        Ok(id) => id,
        Err(e) => {
            println!("Error at converting ObjectId {:?}", e);
            return DbEntry::Error(AcctCodes::InternalError);
        }
    };
    println!("ObjectId to search is: {:?}", oid);
    let db_entries = match get_accounts_list(UserId::ObjectId(oid.clone())) {
        Ok(Some(entry)) => entry,
        Ok(None) => {
            match create_acct_user(oid) {
                Ok(Some(entry)) => entry,
                Ok(None) => {
                    return DbEntry::Error(AcctCodes::NoSuchUser);
                }
                Err(e) => {
                    println!("bError: {:?}", e);
                    return DbEntry::Error(AcctCodes::DatabaseError);
                }
            }
            // return Accounts::Error(AcctCodes::NoSuchUser);
        }
        Err(e) => {
            println!("aError: {:?}", e);
            return DbEntry::Error(AcctCodes::DatabaseError);
        }
    };

    DbEntry::Entry(db_entries)
}

pub(crate) fn get_accounts(claim: Claim) -> Accounts {
    let db_entries = match get_db_entry(&claim) {
        DbEntry::Entry(en) => en,
        DbEntry::Error(code) => return Accounts::Error(code),
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

pub(crate) fn modify_account(
    claim: Claim,
    site: SiteAccount,
    action: SiteAccountAction,
) -> AcctCodes {
    println!("Action is: {:?}", site);

    let mut db_entry = match get_db_entry(&claim) {
        DbEntry::Entry(en) => en,
        DbEntry::Error(code) => return code,
    };
    println!("db found: {:?}", db_entry);

    let entry_type = match env::var("CLEAR") {
        Ok(_) => EntryType::Clear,
        Err(_) => EntryType::Encrypted,
    };
    println!("entry type is: {:?}", entry_type);

    let entries = match entry_type {
        EntryType::Clear => &mut db_entry.clear_entries,
        EntryType::Encrypted => &mut db_entry.entries,
    };

    let id = match bson::oid::ObjectId::with_string(&site.id) {
        Ok(id) => id,
        Err(e) => {
            println!("Error: {:?}", e);
            return AcctCodes::InternalError;
        }
    };

    let db_site = DatabaseSiteAccount {
        id: id.clone(),
        site: site.site,
        username: site.username,
        password: site.password,
        favorite: site.favorite,
    };
    //     entries.push(db_site.clone());
    let mut found_idx: usize = std::usize::MAX;
    for (idx, _) in entries.iter().enumerate() {
        if entries[idx].id.eq(&id) {
            found_idx = idx;
            break;
        }
    }
    let maybe_entry = entries.get_mut(found_idx);

    match action {
        SiteAccountAction::Put => {
            if let Some(entry) = maybe_entry {
                println!("Need to update");
                *entry = db_site;
                if let Err(e) = update_accounts(db_entry) {
                    println!("Error: {:?}", e);
                    return AcctCodes::DatabaseError;
                }
                AcctCodes::AccountChanged
            } else {
                println!("Need to create");
                entries.push(db_site);
                if let Err(e) = update_accounts(db_entry) {
                    println!("Error: {:?}", e);
                    return AcctCodes::DatabaseError;
                }
                AcctCodes::AccountAdded
            }
        }
        SiteAccountAction::Delete => {
            if maybe_entry.is_some() {
                println!("Need to delete");
                entries.remove(found_idx);
                // AcctCodes::AccountDeleted
            } else {
                println!("Nothing to delete");
            }
            if let Err(e) = update_accounts(db_entry) {
                println!("Error: {:?}", e);
                return AcctCodes::DatabaseError;
            }
            AcctCodes::AccountDeleted
        }
    }
}
