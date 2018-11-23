use crate::{schema::accounts, validate::Validator};

use compat_uuid::Uuid;
use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable, Queryable,
};
use postgres_resource::*;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::JsonValue;

#[resource(schema = accounts, table = "accounts")]
struct Account {
    #[optional]
    uuid: Uuid,

    #[optional]
    username: String,

    #[optional]
    password: String,

    #[optional]
    email: String,

    #[optional]
    #[fk]
    verification_id: i32,
}

impl Validator for Account {
    fn validate(&self) -> Result<(), Custom<JsonValue>> {
        if self.email.is_none() {
            return Err(err!(Status::BadRequest, "email required"));
        }

        if self.username.is_none() {
            return Err(err!(Status::BadRequest, "username required"));
        }

        if self.password.is_none() {
            return Err(err!(Status::BadRequest, "password required"));
        }

        Ok(())
    }
}

mod change_password;
mod hash_password;
pub mod route;
mod verify_password;
