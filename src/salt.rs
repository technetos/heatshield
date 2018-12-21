use crate::schema::salts;

use diesel::{
    self, delete, insert_into, prelude::*, result::Error, update, Associations, FromSqlRow,
    Identifiable, Insertable,
};
use postgres_resource::*;

#[resource]
struct Salt {
    salt: String,
}

#[cfg(feature = "gensalt")]
pub mod create_salt;
mod get_salt;
