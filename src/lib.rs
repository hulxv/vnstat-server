#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
use rocket;

pub mod db;
pub mod routes;

pub fn rocket_launcher() -> rocket::Rocket {
    rocket::ignite().mount(
        "/",
        rocket::routes![
            routes::traffic::get_traffic,
            routes::info::get_info,
            routes::config::edit_configs,
            routes::config::get_configs,
            routes::daemon::change_daemon_status,
            routes::daemon::get_daemon_status
        ],
    )
}
