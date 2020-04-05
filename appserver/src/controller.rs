use accounts::accounts_provider::{AccountsControl};
use authenticator::authenticator::{AuthenticatorControl};
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

use crate::utils::{get_request_type, Requests, authorise};

pub struct AppServer<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsControl,
{
    controller: MainRestController<Auth, Acct>,
}

impl<Auth, Acct> AppServer<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsControl,
{
    /// Constructor
    pub fn new(auth: Auth, acct: Acct) -> AppServer<Auth, Acct> {
        AppServer {
            controller: MainRestController::new(auth, acct),
        }
    }

    /// Method to start the server
    /// Binds on localhost:`port` and for every successive
    /// connection takes it to `MainRestController`
    pub fn start(&self, port: u16) -> io::Result<()> {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
        let listener = TcpListener::bind(&addr)?;

        for stream in listener.incoming() {
            self.controller.handle_connection(stream?);
        }

        Ok(())
    }
}

pub struct MainRestController<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsControl,
{
    auth_controller: Auth,
    accounts_controller: Acct,
}

impl<Auth, Acct> MainRestController<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsControl,
{
    /// Constructor
    pub fn new(auth: Auth, acct: Acct) -> MainRestController<Auth, Acct> {
        MainRestController {
            auth_controller: auth,
            accounts_controller: acct,
        }
    }

    /// The main method of communicating with the Frontend for HTTP requests / responses
    pub fn handle_connection(&self, mut stream: TcpStream) {
        println!("Connection established.");
        // This is a mock response, uncomment to skip all other code
        // let response = format!("HTTP/1.1 501 Not Implemented\r\n\r\n");
        // stream.write(response.as_bytes())?;
        // stream.flush().unwrap();

        let mut buffer = [0; 512];
        let implicit_response: String = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n");

        let mut response = implicit_response.clone();

        let ret = stream.read(&mut buffer);
        if ret.is_err() {
            stream.write(implicit_response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }

        let buffer = String::from_utf8_lossy(&buffer[..]);
        let buffer = String::from(buffer);
        let json = buffer.rsplit_terminator("\r\n\r\n").next();
        if json.is_none() {
            stream.write(implicit_response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
        let json = json.unwrap();

        let end_json = json.rfind("}");
        let json: Option<&str> = match end_json {
            Some(idx) => Some(&json[..(idx + 1)]),
            None => None,
        };

        let page = buffer.split_whitespace().nth(1).unwrap();
        println!("{}", page);

        let request: Requests = get_request_type(page);

        // TODO: ADD AUTHENTICATION CHECK, like NOW

        // Big if to branch to specific controller
        if request == Requests::Register {
            response = self.auth_controller.register_user_response(json);
        } else if request == Requests::Authentication {
            response = self.auth_controller.login_response(json);
        } else if request == Requests::ModifyMasterPassword {
            let jwt = authorise(&buffer);
            if jwt.is_none() {
                response = format!("HTTP/1.1 403 Forbidden\r\n\r\n");
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
            response = self.auth_controller.modify_pass_response(json);
        } else if request == Requests::ModifyAccount {
            let jwt = authorise(&buffer);
            if jwt.is_none() {
                response = format!("HTTP/1.1 403 Forbidden\r\n\r\n");
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
            let jwt = jwt.unwrap();
            response = self.accounts_controller.modify_site_account_response(json, jwt);
            // response = self.auth_controller.modify_pass_response(json);
        } else if request == Requests::AccountsList {
            let jwt = authorise(&buffer);
            if jwt.is_none() {
                response = format!("HTTP/1.1 403 Forbidden\r\n\r\n");
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
            let jwt = jwt.unwrap();
            response = self.accounts_controller.get_all_site_accounts_response(jwt);
        }

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
