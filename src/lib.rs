#![recursion_limit = "512"]
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(generic_associated_types)]

pub const BASEPATH: &'static str = "/heatshield/v1";

#[macro_use]
pub mod controller;

pub mod account;
pub mod client;
mod db;
mod granter;
mod policy;

#[cfg(not(feature = "gensalt"))]
mod salt;
#[cfg(feature = "gensalt")]
pub mod salt;

mod refresh_token;
mod sanitize;
mod schema;
pub mod token;
mod user_token;
pub mod validate;
mod verification;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate mashup;
