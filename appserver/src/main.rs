use crate::controller::AppServer;
use crate::utils::*;
use accounts::controller::AccountsRestController;
use authenticator::controller::AuthRestController;
use authenticator::gcp_auth::GcpAuthenticator;
use database::primary_data_provider::PrimaryDataProvider;

mod controller;
mod utils;

fn main() {
    println!("Hello, world!");
    let auth = GcpAuthenticator::new();
    let auth_ctrl = AuthRestController::new(auth);
    let dp = PrimaryDataProvider::new();
    let acct_ctrl = AccountsRestController::new(dp);

    let app = AppServer::new(auth_ctrl, acct_ctrl);

    app.start(8082).unwrap_or_else(|error| {
        eprintln!("{:?}", error);
    })
}
