use crate::{
    client::model::{Client, ClientWithId},
    schema::user_tokens,
};

use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[table_name = "user_tokens"]
pub struct UserTokenWithId {
    pub id: i32,
    pub user_token: UserToken,
}

#[derive(
    Serialize, Deserialize, FromSqlRow, Associations, Insertable, AsChangeset, Debug, PartialEq,
)]
#[table_name = "user_tokens"]
pub struct UserToken {
    pub client_id: Uuid,
    pub account_id: Uuid,
    pub refresh_id: Option<Uuid>,
}

impl Queryable<user_tokens::SqlType, diesel::pg::Pg> for UserTokenWithId {
    type Row = (i32, Uuid, Uuid, Option<Uuid>);
    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            user_token: UserToken {
                client_id: row.1,
                account_id: row.2,
                refresh_id: row.3,
            },
        }
    }
}
