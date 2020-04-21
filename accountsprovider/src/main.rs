#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::appserver::AppServer;

mod accountsprovider;
mod appserver;
mod controller;

fn main() {
    AppServer::run();
}
