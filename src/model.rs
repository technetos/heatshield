use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use schema::{access_tokens, accounts, clients, confirmations, verifications};

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Client)]
pub struct AccessToken {
    pub id: i32,
    pub client_id: Option<i32>,
    pub enabled: Option<bool>,
}

pub type Clientkind = ClientKind;

#[derive(Debug, PartialEq)]
pub enum ClientKind {
    HeatsheildWasm,
}

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct Client {
    pub id: i32,
    pub kind: Option<ClientKind>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[belongs_to(Verification)]
#[table_name = "accounts"]
pub struct AccountWithId {
    pub id: i32,
    pub account: Account,
    pub verification_id: Option<i32>,
}

#[derive(Serialize, Deserialize, FromSqlRow, Insertable, AsChangeset, Debug, PartialEq)]
#[table_name = "accounts"]
pub struct Account {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub enabled: Option<bool>,
}

impl Queryable<accounts::SqlType, diesel::pg::Pg> for AccountWithId {
    type Row = (
        i32,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<bool>,
        Option<i32>,
    );
    fn build(row: Self::Row) -> Self {
        AccountWithId {
            id: row.0,
            account: Account {
                username: row.1,
                password: row.2,
                email: row.3,
                enabled: row.4,
            },
            verification_id: row.5,
        }
    }
}

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
