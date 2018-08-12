#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

pub const BASEPATH: &'static str = "/heatsheild/v1";

pub mod account;
pub mod salt;
pub mod controller;
mod db;
pub mod model;
mod policy;
mod sanitize;
mod schema;
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
