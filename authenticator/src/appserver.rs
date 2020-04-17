use crate::authenticator_control::controller;
// use rocket_cors::{Error, Cors};

pub struct AppServer;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

impl AppServer {
    pub fn run() {
        let cors_options = match rocket_cors::CorsOptions::default().to_cors() {
            Ok(cors) => cors,
            Err(e) => {
                eprintln!("Error at CORS init: {}", e);
                return;
            }
        };

        rocket::ignite()
            .mount(
                "/",
                routes![
                    hello,
                    controller::register,
                    controller::login,
                    controller::modify_password,
                    controller::modify_account_data,
                ],
            )
            .register(catchers![
                controller::already_registered,
                controller::unregistered_user,
            ])
            .attach(cors_options)
            .launch();
    }
}
