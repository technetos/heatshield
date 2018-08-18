use client::model::{Client, ClientWithId};
use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use schema::access_tokens;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[table_name = "access_tokens"]
pub struct AccessTokenWithId {
    pub id: i32,
    pub access_token: AccessToken,
}

#[derive(
    Serialize, Deserialize, FromSqlRow, Associations, Insertable, AsChangeset, Debug, PartialEq,
)]
#[belongs_to(Client)]
#[table_name = "access_tokens"]
pub struct AccessToken {
    pub client_id: Uuid,
}

impl Queryable<access_tokens::SqlType, diesel::pg::Pg> for AccessTokenWithId {
    type Row = (i32, Uuid);
    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            access_token: AccessToken { client_id: row.1 },
        }
    }
}
