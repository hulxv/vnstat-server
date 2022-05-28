#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate actix_web;

// modules
pub mod server;
// pub mod utils;

pub use server::{api, http};
