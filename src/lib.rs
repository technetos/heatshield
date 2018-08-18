#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(generic_associated_types)]

pub const BASEPATH: &'static str = "/heatshield/v1";

pub mod account;
pub mod client;
pub mod controller;
mod db;
pub mod model;
mod policy;
pub mod salt;
mod sanitize;
mod schema;
pub mod token;
mod validate;

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate chrono;

#[macro_use]
extern crate diesel;

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate data_encoding;

extern crate ring;

extern crate uuid;

extern crate jsonwebtoken;
