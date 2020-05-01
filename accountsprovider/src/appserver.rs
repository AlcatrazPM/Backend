use crate::controller;

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
                    controller::get_accounts,
                    controller::add_account,
                    controller::modify_account,
                    controller::delete_account,
                ],
            )
            .attach(cors_options)
            .launch();
    }
}
