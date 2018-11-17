use crate::{salt::SaltController, schema};

use diesel::prelude::*;
use postgres_resource::ResourceController;

impl SaltController {
    pub fn get_salt(&self, username: &str) -> Result<Vec<u8>, ()> {
        let db_salt = self
            .get_one(Box::new(schema::salts::id.eq(1)))
            .map_err(|_| panic!("No salt found"))?
            .salt
            .salt;

        let salt_bytes = db_salt.as_bytes();
        let username_bytes = username.as_bytes();

        let mut res = Vec::with_capacity(username_bytes.len() + salt_bytes.len());
        res.extend(salt_bytes);
        res.extend(username_bytes);

        Ok(res)
    }
}
