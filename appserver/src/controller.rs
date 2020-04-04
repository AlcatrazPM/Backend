use accounts::accounts_provider::AccountsProvider;
use authenticator::authenticator::Authenticator;
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, SocketAddrV4, Ipv4Addr};
use std::io;
use std::io::{Read, Write};

// extern crate http;
// use http::{Response, StatusCode};

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
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port
        );
        let listener = TcpListener::bind(&addr)?;

        for stream in listener.incoming() {
            self.controller.handle_connection(stream?)?;
        }

        Ok(())
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
    /// Constructor
    pub fn new(auth: Auth, acct: Acct) -> MainRestController<Auth, Acct> {
        MainRestController {
            auth_controller: auth,
            accounts_controller: acct,
        }
    }

    /// The main method of communicating with the Frontend for HTTP requests / responses
    pub fn handle_connection(&self, mut stream: TcpStream) -> io::Result<()> {
        println!("Connection established.");

        let mut buffer = [0; 512];

        stream.read(&mut buffer)?;

        let buffer = String::from_utf8_lossy(&buffer[..]);

        let request: Vec<&str> = buffer.split_terminator("\r\n").collect();

        for line in request.iter() {
            println!("{}", line);
        }

        // TODO: parse request, choose controller to work and build & send response

        // This is a mock response
        let response = format!("HTTP/1.1 501 Not Implemented\r\n\r\n");
        stream.write(response.as_bytes())?;
        stream.flush().unwrap();

        Ok(())
    }
}
