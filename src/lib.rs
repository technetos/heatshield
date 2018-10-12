#![recursion_limit = "512"]
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(generic_associated_types)]
#![feature(tool_attributes)]

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

#[macro_use]
extern crate mashup;
