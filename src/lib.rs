#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
extern crate actix_web;
use actix_web::{web, App, HttpServer};
use std::io::{self, Write};

// modules
pub mod app;
pub mod http;
pub mod routes;
pub mod utils;
pub mod vnstat;

#[actix_web::main]
pub async fn run_server() -> anyhow::Result<()> {
    let configs = app::config::Configs::init()?;
    let (ip, port) = (configs.server.ip, configs.server.port as u16);

    io::stdout().flush().unwrap();
    println!("Server launched on {ip}:{port}");

    match HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(
                web::scope("/vnstat")
                    .service(routes::traffic::get_traffic)
                    .service(routes::interface::get_interfaces)
                    .service(routes::info::get_info)
                    .service(routes::config::get_config),
            ),
        )
    })
    .bind((ip, port))?
    .run()
    .await
    {
        Err(err) => return Err(anyhow::anyhow!(err)),
        _ => (),
    };
    Ok(())
}
