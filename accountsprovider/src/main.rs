#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::appserver::AppServer;

mod appserver;
mod controller;
mod accountsprovider;

fn main() {
    AppServer::run();
}
