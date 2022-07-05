#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

// modules
pub mod cli;
pub mod server;
pub mod uds_request_handler;

pub use cli::Args;
pub use server::{api, http};
