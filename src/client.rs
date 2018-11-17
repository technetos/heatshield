use crate::{schema::clients, validate::Validator};

use diesel::{
    self, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow, Identifiable,
    Insertable, Queryable,
};
use postgres_resource::*;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::JsonValue;
use compat_uuid::Uuid;

#[resource(schema = clients, table = "clients")]
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
