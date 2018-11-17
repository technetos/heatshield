use crate::schema::user_tokens;

use diesel::{
    self, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow, Identifiable,
    Insertable, Queryable,
};
use postgres_resource::*;
use compat_uuid::Uuid;

#[resource(schema = user_tokens, table = "user_tokens")]
struct UserToken {
    client_id: Uuid,

    account_id: Uuid,

    #[optional]
    refresh_id: Uuid,
}
