use crate::controller::AppServer;
use accounts::acct_provider::AcctProvider;
use accounts::controller::AccountsRestController;
use authenticator::controller::AuthRestController;
use authenticator::gcp_auth::GcpAuthenticator;
use database::primary_data_provider::PrimaryDataProvider;

mod controller;
mod utils;

fn main() {
    println!("Hello, world!");
    let dp = PrimaryDataProvider::new();
    let auth = GcpAuthenticator::new(&dp);
    let acct = AcctProvider::new(&dp);
    let auth_ctrl = AuthRestController::new(auth);
    let acct_ctrl = AccountsRestController::new(acct);

    let app = AppServer::new(auth_ctrl, acct_ctrl);

    app.start(8082).unwrap_or_else(|error| {
        eprintln!("{:?}", error);
    })
}
