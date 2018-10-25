use crate::{schema::clients, validate::Validator};

use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use rocket_contrib::{Json, Value};
use rocket::{response::status::Custom, http::Status};
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
    pub uuid: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Validator for Client {
    fn validate(&self) -> Result<(), Custom<Json>> {
        if self.name.is_none() {
            return Err(Custom(Status::BadRequest, Json(json!("name required"))));
        }

        if self.email.is_none() {
            return Err(Custom(Status::BadRequest, Json(json!("email required"))));
        }

        Ok(())
    }
}

impl Queryable<clients::SqlType, diesel::pg::Pg> for ClientWithId {
    type Row = (i32, Uuid, Option<String>, Option<String>);
    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            client: Client {
                uuid: row.1,
                name: row.2,
                email: row.3,
            },
        }
    }
}
