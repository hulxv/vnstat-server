#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate actix_web;

// modules
pub mod cli;
pub mod server;

pub use cli::Args;
pub use server::{api, http};
