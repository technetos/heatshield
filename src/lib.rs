#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

pub mod account_controller;
pub mod controller;
mod db;
pub mod model;
pub mod router;
mod schema;

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

extern crate failure;
