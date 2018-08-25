use client::model::{Client, ClientWithId};
use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use schema::client_tokens;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[table_name = "client_tokens"]
pub struct ClientTokenWithId {
    pub id: i32,
    pub client_token: ClientToken,
}

#[derive(
    Serialize, Deserialize, FromSqlRow, Associations, Insertable, AsChangeset, Debug, PartialEq,
)]
#[belongs_to(Client)]
#[table_name = "client_tokens"]
pub struct ClientToken {
    pub client_id: Uuid,
}

impl Queryable<client_tokens::SqlType, diesel::pg::Pg> for ClientTokenWithId {
    type Row = (i32, Uuid);
    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            client_token: ClientToken { client_id: row.1 },
        }
    }
}
