use crate::schema::salts;

use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable, Queryable,
};
use postgres_resource::*;

#[resource(schema = salts, table = "salts")]
struct Salt {
    salt: String,
}

#[cfg(feature = "gensalt")]
pub mod create_salt;
mod get_salt;
