use crate::schema::refresh_tokens;

use compat_uuid::Uuid;
use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable, Queryable,
};
use postgres_resource::*;

#[resource(schema = refresh_tokens, table = "refresh_tokens")]
struct RefreshToken {
    uuid: Uuid,
}
