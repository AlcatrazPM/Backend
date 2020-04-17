#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use(bson, doc)]
extern crate bson;

use crate::appserver::AppServer;

mod appserver;
mod authenticator_control;
mod dataprovider;
mod jwt;

fn main() {
    AppServer::run();
}
