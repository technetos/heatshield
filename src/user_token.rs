use crate::schema::user_tokens;

use compat_uuid::Uuid;
use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable, Queryable,
};
use postgres_resource::*;

#[resource(schema = user_tokens, table = "user_tokens")]
struct UserToken {
    client_id: Uuid,

    account_id: Uuid,

    #[optional]
    refresh_id: Uuid,
}
