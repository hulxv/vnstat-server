#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
extern crate actix_web;
use actix_web::{App, HttpServer};

pub mod app;
pub mod http;
pub mod routes;
pub mod utils;
pub mod vnstat;

#[actix_web::main]
pub async fn run_server() -> std::io::Result<()> {
    println!("Server launched on 127.0.0.1:8080");
    match HttpServer::new(|| {
        App::new()
            .service(routes::traffic::get_traffic)
            .service(routes::interface::get_interfaces)
            .service(routes::info::get_info)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    {
        Err(err) => eprintln!("{err}"),
        _ => (),
    }
    Ok(())
}
