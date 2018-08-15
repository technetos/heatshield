use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use schema::clients;
use rocket_contrib::{Json, Value};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[table_name = "clients"]
pub struct ClientWithId {
    pub id: i32,
    pub client: Client,
}

#[derive(Serialize, Deserialize, FromSqlRow, Insertable, AsChangeset, Debug, PartialEq)]
#[table_name = "clients"]
pub struct Client {
    pub uuid: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
          uuid: None,
          name: None,
          email: None,
        }
    }
}

impl Queryable<clients::SqlType, diesel::pg::Pg> for ClientWithId {
    type Row = (i32, Option<Uuid>, Option<String>, Option<String>);
    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            client: Client {
              uuid: row.1
              name: row.2,
              email: row.3,
            },
        }
    }
}

