#![feature(custom_attribute)]
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

macro_rules! err {
    ($status:expr, $msg:expr) => {
        Custom($status, Json(json!({ "error_message": $msg })))
    };
}

mod result {
    use rocket::response::status::Custom;
    use rocket_contrib::Json;
    pub type WebResult = std::result::Result<Json, Custom<Json>>;
}

pub const BASEPATH: &'static str = "/heatshield/v1";

#[macro_use]
extern crate postgres_resource;

pub mod account;
pub mod client;
mod granter;
mod policy;

#[cfg(not(feature = "gensalt"))]
pub mod salt;
#[cfg(feature = "gensalt")]
pub mod salt;

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
