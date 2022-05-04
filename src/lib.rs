#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
extern crate actix_web;

// modules
pub mod app;
pub mod server;
pub mod utils;
pub mod vnstat;

pub use server::{api, http};
