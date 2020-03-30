use authenticator::gcp_auth::GcpAuthenticator;
use database::primary_data_provider::PrimaryDataProvider;
use accounts::controller::AccountsRestController;
use crate::controller::AppServer;


mod controller;

fn main() {
    println!("Hello, world!");
    let auth = GcpAuthenticator::new();
    let dp = PrimaryDataProvider::new();
    let acct_ctrl = AccountsRestController::new(dp);

    let _app = AppServer::new(auth, acct_ctrl);
}
