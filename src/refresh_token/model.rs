use crate::{
    client::model::{Client, ClientWithId},
    schema::refresh_tokens,
};

use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[table_name = "refresh_tokens"]
pub struct RefreshTokenWithId {
    pub id: i32,
    pub refresh_token: RefreshToken,
}

#[derive(
    Serialize, Deserialize, FromSqlRow, Associations, Insertable, AsChangeset, Debug, PartialEq,
)]
#[table_name = "refresh_tokens"]
pub struct RefreshToken {
    pub uuid: Uuid,
}

impl Queryable<refresh_tokens::SqlType, diesel::pg::Pg> for RefreshTokenWithId {
    type Row = (i32, Uuid);
    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            refresh_token: RefreshToken { uuid: row.1 },
        }
    }
}
