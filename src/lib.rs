#![feature(proc_macro_hygiene, decl_macro)]

macro_rules! err {
    ($status:expr, $msg:expr) => {
        Custom($status, json!({ "error_message": $msg }))
    };
}

mod result {
    use rocket::response::status::Custom;
    use rocket_contrib::json::JsonValue;
    pub type WebResult = std::result::Result<JsonValue, Custom<JsonValue>>;
}

pub const BASEPATH: &'static str = "/heatshield/v1";

#[macro_use]
extern crate postgres_resource;

pub mod account;
pub mod client;
mod granter;
mod jwt;
pub mod logout;
mod policy;

#[cfg(not(feature = "gensalt"))]
pub mod salt;
#[cfg(feature = "gensalt")]
pub mod salt;

mod access_token;
mod refresh_token;
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
