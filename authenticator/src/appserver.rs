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
                ],
            )
            .register(catchers![
                controller::unregistered_user,
                controller::already_registered,
            ])
            .launch();
    }
}
