use crate::{schema::clients, validate::Validator};

use compat_uuid::Uuid;
use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable,
};
use postgres_resource::*;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::JsonValue;

#[resource]
struct Client {
    uuid: Uuid,

    #[optional]
    name: String,

    #[optional]
    email: String,
}

impl Validator for Client {
    fn validate(&self) -> Result<(), Custom<JsonValue>> {
        if self.name.is_none() {
            return Err(Custom(Status::BadRequest, json!("name required")));
        }

        if self.email.is_none() {
            return Err(Custom(Status::BadRequest, json!("email required")));
        }

        Ok(())
    }
}

pub mod route;
