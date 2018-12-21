use crate::schema::access_tokens;

use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable,
};
use postgres_resource::*;

#[resource]
struct AccessToken {
    jwt: String,

    expires_in: i32,

    user_id: i32,
}
