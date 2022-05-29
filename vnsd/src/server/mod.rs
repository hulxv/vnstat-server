pub mod api;
pub mod http;

use actix_web::{dev::Service, middleware::Logger, web, App, HttpServer};
use anyhow::anyhow;
use api::routes;
use log::info;
use std::{
    io::{self, Result, Write},
    ops::DerefMut,
};

use app;
pub struct Server {
    ip: String,
    port: u16,
}

impl Server {
    pub fn new() -> ServerBuilder {
        ServerBuilder::new()
    }

    #[actix_web::main]
    pub async fn run(&self) -> Result<()> {
        match HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "[%s] (%r %a) \n  ip: %{r}a\n  time: %Ts,\n  pid: %P,\n  user-agent: %{User-Agent}i,\n  content-type: %{Content-Type}i,\n  size: %bb",
            ))
            .service(
                web::scope("/api")
                    .service(routes::traffic::get_traffic)
                    .service(routes::interface::get_interface)
                    .service(routes::info::get_info)
                    .service(routes::config::get_config),
            )
    })
    .bind((self.ip.as_str().clone().to_owned(),self.port.clone()))
    {
        Err(err) => Err(err),
        Ok(server) => {
            info!("Server running on http://{}:{} ", self.ip, self.port);
            server.run().await?;
            Ok(())
        }
    }
    }
}

impl Default for Server {
    fn default() -> Self {
        let server_builder = ServerBuilder::new();
        server_builder.from_config_file().build()
    }
}

pub struct ServerBuilder {
    ip: Option<String>,
    port: Option<u16>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            ip: None,
            port: None,
        }
    }

    pub fn ip(&mut self, ip: &str) -> &mut Self {
        self.ip = Some(ip.to_owned());
        self
    }
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub fn from_config_file(&self) -> Self {
        let configs = app::config::Configs::init().unwrap();
        let (ip, port) = (configs.server.ip, configs.server.port as u16);
        Self {
            ip: Some(ip),
            port: Some(port),
        }
    }

    pub fn build(&self) -> Server {
        Server {
            ip: self.ip.as_ref().unwrap().to_string(),
            port: self.port.unwrap(),
        }
    }
}
