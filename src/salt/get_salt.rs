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

        let mut res = Vec::with_capacity(username.as_bytes().len() + db_salt.as_bytes().len());

        res.extend(db_salt.as_bytes());
        res.extend(username.as_bytes());

        Ok(res)
    }
}
