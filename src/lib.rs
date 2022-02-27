#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
use rocket::{
    config::{Config, Environment},
    fairing::AdHoc,
};
use std::io::Write;

pub mod app;
pub mod db;
pub mod routes;
pub mod utils;

pub fn rocket_launcher() -> rocket::Rocket {
    let server_config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8800)
        .finalize()
        .unwrap();

    rocket::custom(server_config).mount(
        "/",
        rocket::routes![
            routes::traffic::get_traffic,
            routes::info::get_info,
            routes::config::edit_configs,
            routes::config::get_configs,
            routes::daemon::change_daemon_status,
            routes::daemon::get_daemon_status,
            routes::interface::get_interfaces
        ],
    )
}
