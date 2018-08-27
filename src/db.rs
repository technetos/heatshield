use controller::ResourceController;
use data_encoding;
use data_encoding::HEXUPPER;
use diesel;
use diesel::ExpressionMethods;
use diesel::{pg::PgConnection, Connection};
use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};
use salt::{
    controller::SaltController,
    model::{Salt, SaltWithId},
};
use schema;
use std::{env, error::Error};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

pub fn establish_connection() -> PgConnection {
    let env_var = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&env_var[..]).unwrap()
}

fn salt_component() -> Result<String, diesel::result::Error> {
    Ok(SaltController
        .get_one(Box::new(schema::salts::id.eq(1)))?
        .salt
        .salt)
}

pub fn salt(username: &str) -> Result<Vec<u8>, ()> {
    let db_salt = salt_component().map_err(|_| panic!("No salt found"))?;
    let mut res = Vec::with_capacity(username.as_bytes().len() + db_salt.as_bytes().len());

    res.extend(db_salt.as_bytes());
    res.extend(username.as_bytes());

    Ok(res)
}
