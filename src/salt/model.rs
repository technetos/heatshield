use crate::schema::salts;

use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[table_name = "salts"]
pub struct SaltWithId {
    pub id: i32,
    pub salt: Salt,
}

#[derive(Serialize, Deserialize, FromSqlRow, Insertable, AsChangeset, Debug, PartialEq)]
#[table_name = "salts"]
pub struct Salt {
    pub salt: String,
}

impl Queryable<salts::SqlType, diesel::pg::Pg> for SaltWithId {
    type Row = (i32, String);
    fn build(row: Self::Row) -> Self {
        SaltWithId {
            id: row.0,
            salt: Salt { salt: row.1 },
        }
    }
}
