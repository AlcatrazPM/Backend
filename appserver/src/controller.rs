use accounts::accounts_provider::AccountsProvider;
use authenticator::authenticator::Authenticator;

pub struct AppServer<Auth, Acct>
    where
        Auth: Authenticator,
        Acct: AccountsProvider,
{
    controller: MainRestController<Auth, Acct>,
}

impl<Auth, Acct> AppServer<Auth, Acct>
    where
        Auth: Authenticator,
        Acct: AccountsProvider,
{
    pub fn new(auth: Auth, acct: Acct) -> AppServer<Auth, Acct> {
        AppServer {
            controller: MainRestController::new(auth, acct),
        }
    }
}

pub struct MainRestController<Auth, Acct>
where
    Auth: Authenticator,
    Acct: AccountsProvider,
{
    auth_controller: Auth,
    accounts_controller: Acct,
}

impl<Auth, Acct> MainRestController<Auth, Acct>
where
    Auth: Authenticator,
    Acct: AccountsProvider,
{
    pub fn new(auth: Auth, acct: Acct) -> MainRestController<Auth, Acct> {
        MainRestController {
            auth_controller: auth,
            accounts_controller: acct,
        }
    }
}
