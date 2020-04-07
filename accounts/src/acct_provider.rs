use crate::accounts_provider::{AccountsCodes, AccountsProvider};
use database::data_provider::DataProvider;
use userdata::siteinfo::SiteAccount;

pub struct AcctProvider<'a, DP: DataProvider> {
    dataprovider: &'a DP,
}

impl<DP> AcctProvider<'_, DP>
    where
        DP: DataProvider,
{
    pub fn new(dataprovider: &DP) -> AcctProvider<DP> {
        AcctProvider { dataprovider }
    }
}

impl<DP> AccountsProvider for AcctProvider<'_, DP>
    where
        DP: DataProvider,
{
    fn add_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes {
        println!("add to user {} site {:?}", user_id, site);
        self.dataprovider.save_logs("added new site".to_string());
        AccountsCodes::OperationOK
    }

    fn remove_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes {
        println!("remove from user {} site {:?}", user_id, site);
        AccountsCodes::OperationOK
    }

    fn modify_site_account(&self, user_id: String, site: SiteAccount) -> AccountsCodes {
        println!("modify user {} site {:?}", user_id, site);
        AccountsCodes::OperationOK
    }

    fn get_all_site_accounts(&self, user_id: String) -> (Vec<SiteAccount>, AccountsCodes) {
        println!("List sites for user {}", user_id);
        let sites = test_sites();

        (sites, AccountsCodes::OperationOK)
        // (vec![], AccountsCodes::NotImplemented)
    }
}

fn test_sites() -> Vec<SiteAccount> {
    let s1 = SiteAccount {
        username: String::from("a"),
        password: String::from("b"),
        id: String::from("id1"),
    };
    let s2 = SiteAccount {
        username: String::from("c"),
        password: String::from("d"),
        id: String::from("id2"),
    };
    let s3 = SiteAccount {
        username: String::from("x"),
        password: String::from("y"),
        id: String::from("id3"),
    };

    vec![s1, s2, s3]
}
