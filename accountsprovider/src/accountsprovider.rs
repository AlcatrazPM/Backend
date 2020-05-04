use dataprovider::data_structs::{DatabaseAccountEntry, DatabaseSiteAccount, UserId};
use dataprovider::primary_data_provider::{create_acct_user, get_accounts_list, update_accounts};
use jwt::claim::Claim;
use std::env;
use userdata::userdata::{
    Accounts, AccountsList, AcctCodes, AddSite, IdSite, ReturnIdSite, SiteAccount,
};

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

pub(crate) fn add_account(claim: Claim, site: AddSite) -> ReturnIdSite {
    println!("Site to add is: {:?}", site);

    let mut db_entry = match get_db_entry(&claim) {
        DbEntry::Entry(en) => en,
        DbEntry::Error(code) => return ReturnIdSite::Error(code),
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

    let id = match bson::oid::ObjectId::new() {
        Ok(id) => id,
        Err(e) => {
            println!("Error: {:?}", e);
            return ReturnIdSite::Error(AcctCodes::InternalError);
        }
    };

    entries.push(DatabaseSiteAccount {
        id: id.clone(),
        site: site.site,
        username: site.username,
        password: site.password,
        favorite: site.favorite,
    });

    if let Err(e) = update_accounts(db_entry) {
        println!("Error: {:?}", e);
        return ReturnIdSite::Error(AcctCodes::DatabaseError);
    }
    // AcctCodes::AccountAdded
    ReturnIdSite::Id(IdSite { id: id.to_hex() })
}

pub(crate) fn modify_account(claim: Claim, site: SiteAccount) -> AcctCodes {
    println!("Site to modify is: {:?}", site);
    apply_action(claim, site.id.clone(), Action::Update(site))
}

pub(crate) fn delete_account(claim: Claim, id: IdSite) -> AcctCodes {
    println!("id to delete site is: {:?}", id);
    apply_action(claim, id.id, Action::Delete)
}

fn apply_action(claim: Claim, id: String, action: Action) -> AcctCodes {
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

    let search_id = match bson::oid::ObjectId::with_string(&id) {
        Ok(id) => id,
        Err(e) => {
            println!("Error: {:?}", e);
            return AcctCodes::InternalError;
        }
    };

    let mut found_idx: usize = std::usize::MAX;
    for (idx, _) in entries.iter().enumerate() {
        if entries[idx].id.eq(&search_id) {
            found_idx = idx;
            break;
        }
    }
    let entry = match entries.get_mut(found_idx) {
        Some(entry) => entry,
        None => {
            println!("No Account found");
            return AcctCodes::AccountNotFound;
        }
    };

    match action {
        Action::Update(site) => {
            println!("Need to update");
            *entry = DatabaseSiteAccount {
                id: search_id,
                site: site.site,
                username: site.username,
                password: site.password,
                favorite: site.favorite,
            };
            if let Err(e) = update_accounts(db_entry) {
                println!("Error: {:?}", e);
                return AcctCodes::DatabaseError;
            }
            AcctCodes::AccountChanged
        }
        Action::Delete => {
            println!("Need to delete");
            entries.remove(found_idx);
            if let Err(e) = update_accounts(db_entry) {
                println!("Error: {:?}", e);
                return AcctCodes::DatabaseError;
            }
            AcctCodes::AccountDeleted
        }
    }
}

enum Action {
    Update(SiteAccount),
    Delete,
}
