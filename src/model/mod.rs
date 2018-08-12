use chrono::NaiveDateTime;
use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use schema::{access_tokens, accounts, clients, confirmations, salts, verifications};

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Client)]
#[table_name = "access_tokens"]
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
#[table_name = "clients"]
pub struct Client {
    pub id: i32,
    pub kind: Option<ClientKind>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub enabled: Option<bool>,
}

pub mod account;

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

#[derive(Queryable, Serialize)]
pub struct Salt {
    pub id: i32,
    pub salt: String,
}
