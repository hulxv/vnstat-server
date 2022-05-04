#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
extern crate actix_web;
use actix_web::{dev::Service, middleware::Logger, web, App, HttpServer};
use anyhow::anyhow;
use log::info;
use std::io::{self, Result, Write};
// modules
pub mod api;
pub mod app;
pub mod http;
pub mod utils;
pub mod vnstat;

use api::routes;

#[actix_web::main]
pub async fn run_server() -> Result<()> {
    let configs = app::config::Configs::init().unwrap();
    let addr = (configs.server.ip, configs.server.port as u16);

    match HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "[%s] (%r %a) \n  time: %Ts,\n  pid: %P,\n  user-agent: %{User-Agent}i,\n  content-type: %{Content-Type}i,\n  size: %bb",
            ))
            .service(
                web::scope("/api")
                    .service(routes::traffic::get_traffic)
                    .service(routes::interface::get_interfaces)
                    .service(routes::info::get_info)
                    .service(routes::config::get_config),
            )
    })
    .bind(addr.clone())
    {
        Err(err) => Err(err),
        Ok(server) => {
            info!("Server running on http://{}:{} ", addr.0, addr.1);
            server.run().await?;
            Ok(())
        }
    }
}
