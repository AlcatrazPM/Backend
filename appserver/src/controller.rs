use accounts::accounts_provider::AccountsProvider;
use authenticator::authenticator::{AuthCodes, Authenticator, AuthenticatorControl, Token};
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

use crate::utils::{get_request_type, Requests};

pub struct AppServer<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsProvider,
{
    controller: MainRestController<Auth, Acct>,
}

impl<Auth, Acct> AppServer<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsProvider,
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
    Acct: AccountsProvider,
{
    auth_controller: Auth,
    accounts_controller: Acct,
}

impl<Auth, Acct> MainRestController<Auth, Acct>
where
    Auth: AuthenticatorControl,
    Acct: AccountsProvider,
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

        stream.read(&mut buffer).unwrap();

        let buffer = String::from_utf8_lossy(&buffer[..]);
        let json = buffer.rsplit_terminator("\r\n\r\n").next().unwrap();
        let end_json = json.rfind("}").unwrap();
        let json = &json[..(end_json + 1)];
        // println!("{}", json);

        // TODO: parse request, choose controller to work and build & send response

        let page = buffer.split_whitespace().nth(1).unwrap();
        println!("{}", page);

        let request: Requests = get_request_type(page);

        let mut response: String = format!("HTTP/1.1 501 Not Implemented\r\n\r\n");
        // Big if to branch to specific controller
        if request == Requests::Register {
            response = self.auth_controller.register_user_response(json);
        } else if request == Requests::Authentication {
            response = self.auth_controller.login_response(json);
        } else if request == Requests::ModifyMasterPassword {
            response = self.auth_controller.modify_pass_response(json);
        }

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
