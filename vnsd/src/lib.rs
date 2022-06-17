#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

// modules
pub mod cli;
pub mod server;

pub use cli::Args;
pub use server::{api, http};
