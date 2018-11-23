use crate::schema::access_tokens;

use compat_uuid::Uuid;
use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable, Queryable,
};
use postgres_resource::*;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::JsonValue;

#[resource(schema = access_tokens, table = "access_tokens")]
struct AccessToken {
    jwt: String,

    expires_in: i32,

    user_id: i32,
}
