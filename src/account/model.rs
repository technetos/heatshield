use chrono::NaiveDateTime;
use data_encoding;
use diesel::{self, Associations, FromSqlRow, Identifiable, Insertable, Queryable};
use verification::model::Verification;
use rocket_contrib::{Json, Value};
use schema::{access_tokens, accounts, clients, confirmations, verifications};
use uuid::Uuid;
use salt::model::Salt;
use validate::Validator;

#[derive(Serialize, Deserialize, FromSqlRow, Associations, Identifiable, Debug, PartialEq)]
#[belongs_to(Verification)]
#[table_name = "accounts"]
pub struct AccountWithId {
    pub id: i32,
    pub account: Account,
    pub verification_id: Option<i32>,
}

use data_encoding::HEXUPPER;
use db;
use ring::{digest, pbkdf2};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

impl AccountWithId {
    pub fn verify_password(&self, current_pw: &str) -> bool {
        let pw_salt = db::salt(self.account.email.as_ref().unwrap()).unwrap();
        let mut actual: Credential = [0u8; CREDENTIAL_LEN];

        pbkdf2::derive(
            DIGEST_ALG,
            100_000,
            &pw_salt,
            current_pw.to_owned().as_bytes(),
            &mut actual,
        );
        let actual_hash = HEXUPPER.encode(&actual);

        Some(&actual_hash) == self.account.password.as_ref()
    }
}

#[derive(Serialize, Deserialize, FromSqlRow, Insertable, AsChangeset, Debug, PartialEq)]
#[table_name = "accounts"]
pub struct Account {
    pub uuid: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

impl Account {
    pub fn new() -> Self {
        Self {
            uuid: None,
            username: None,
            password: None,
            email: None,
        }
    }

    pub fn hash_password(&mut self) {
        let salt = db::salt(&self.email.as_ref().unwrap().clone()).unwrap();
        let mut hash_result: Credential = [0u8; CREDENTIAL_LEN];

        pbkdf2::derive(
            DIGEST_ALG,
            100_000,
            &salt,
            self.password.as_ref().unwrap().clone().as_bytes(),
            &mut hash_result,
        );

        self.password = Some(data_encoding::HEXUPPER.encode(&hash_result));
    }
}

impl Validator for Account {
    fn validate(&self) -> Result<(), Json> {
        if self.email.is_none() {
            return Err(Json(json!("email required")));
        }

        if self.username.is_none() {
            return Err(Json(json!("username required")));
        }

        if self.password.is_none() {
            return Err(Json(json!("password required")));
        }

        Ok(())
    }
}

impl Queryable<accounts::SqlType, diesel::pg::Pg> for AccountWithId {
    type Row = (
        i32,
        Option<Uuid>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i32>,
    );
    fn build(row: Self::Row) -> Self {
        AccountWithId {
            id: row.0,
            account: Account {
                uuid: row.1,
                username: row.2,
                password: row.3,
                email: row.4,
            },
            verification_id: row.5,
        }
    }
}
