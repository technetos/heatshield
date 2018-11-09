use crate::schema::refresh_tokens;

use diesel::{
    self, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow, Identifiable,
    Insertable, Queryable,
};
use postgres_resource::*;
use uuid::Uuid;

#[resource(schema = refresh_tokens, table = "refresh_tokens")]
struct RefreshToken {
    uuid: Uuid,
}
