use chrono::NaiveDateTime;
use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use schema::{confirmations, verifications};

#[derive(Insertable, Queryable, Associations, Identifiable, Debug, PartialEq)]
#[belongs_to(Confirmation)]
pub struct Verification {
    pub id: i32,
    pub verified_at: Option<NaiveDateTime>,
    pub ip_address: Option<String>,
    pub confirmation_id: Option<i32>,
}

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
pub struct Confirmation {
    pub id: i32,
    pub code: Option<String>,
}
