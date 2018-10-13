use crate::schema::verifications;

use chrono::NaiveDateTime;
use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
pub struct Verification {
    pub id: i32,
    pub verified_at: Option<NaiveDateTime>,
    pub ip_address: Option<String>,
}
