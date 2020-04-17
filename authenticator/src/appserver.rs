use crate::authenticator_control::controller;

pub struct AppServer;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

impl AppServer {
    pub fn run() {
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
            .launch();
    }
}
