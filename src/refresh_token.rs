use crate::schema::refresh_tokens;

use compat_uuid::Uuid;
use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable,
};
use postgres_resource::*;

#[resource]
struct RefreshToken {
    uuid: Uuid,
}
