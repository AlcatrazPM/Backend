use accounts::accounts_provider::AccountsControl;
use authenticator::authenticator::AuthenticatorControl;
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

use crate::utils::{authorise, get_request_type, Requests};
use http::response::Response;
use http::statuscode::StatsCodes;

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
        let implicit_response = Response::build().status(StatsCodes::InternalError);

        if stream.read(&mut buffer).is_err() {
            self.write_stream(stream, implicit_response);
            return;
        }


        // don't @me for this
        let buffer = String::from(String::from_utf8_lossy(&buffer[..]));
        println!("{}", buffer);

        let body: &str = match buffer.rsplit_terminator("\r\n\r\n").next() {
            Some(data) => data,
            None => {
                self.write_stream(stream, Response::build().status(StatsCodes::BadRequest));
                return;
            }
        };

        let end_json = body.rfind("}");
        let json: Option<&str> = match end_json {
            Some(idx) => Some(&body[..(idx + 1)]),
            None => None,
        };

        let page = match buffer.split_whitespace().nth(1) {
            Some(p) => p,
            None => {
                self.write_stream(stream, Response::build().status(StatsCodes::BadRequest));
                return;
            }
        };
        println!("{}", page);

        let jwt = authorise(&buffer);
        let request: Requests = get_request_type(page);

        let response: Response = match request {
            Requests::Register => self.auth_controller.register_user_response(json),
            Requests::Authentication => self.auth_controller.login_response(json),
            Requests::ModifyMasterPassword => {
                if jwt.is_none() {
                    self.write_stream(stream, Response::build().status(StatsCodes::Forbidden));
                    return;
                }
                self.auth_controller.modify_pass_response(json)
            }
            Requests::AccountsList => {
                if jwt.is_none() {
                    self.write_stream(stream, Response::build().status(StatsCodes::Forbidden));
                    return;
                }
                self.accounts_controller
                    .get_all_site_accounts_response(jwt.unwrap())
            }
            Requests::ModifyAccount => {
                if jwt.is_none() {
                    self.write_stream(stream, Response::build().status(StatsCodes::Forbidden));
                    return;
                }
                self.accounts_controller
                    .modify_site_account_response(json, jwt.unwrap())
            }
            Requests::BadRequest => {
                self.write_stream(stream, Response::build().status(StatsCodes::Forbidden));
                return;
            }
            Requests::NotImplemented => {
                self.write_stream(stream, Response::build().status(StatsCodes::Forbidden));
                return;
            }
        };

        self.write_stream(stream, response);
    }

    fn write_stream(&self, mut stream: TcpStream, response: Response) {
        if let Err(_) = stream.write(response.to_string().as_bytes()) {
            if let Err(_) = stream.write(response.to_string().as_bytes()) {
                return;
            }
        }
        if let Err(_) = stream.flush() {
            if let Err(_) = stream.flush() {
                return;
            }
        }
    }
}
