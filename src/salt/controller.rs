use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use salt::model::{Salt, SaltWithId};
use schema::salts;

pub struct SaltController;

impl ResourceWithId for SaltController {
    type ModelWithId = SaltWithId;
}

impl Resource for SaltController {
    type Model = Salt;
}

impl ResourceTable for SaltController {
    type DBTable = salts::table;
}

impl ResourceSql for SaltController {
    type SQLType = salts::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for SaltController {
    fn create(&self, model: &Salt) -> Result<SaltWithId, Error> {
        Ok(insert_into(salts::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<salts::table>) -> Result<SaltWithId, Error> {
        Ok(salts::table
            .filter(by)
            .get_result::<SaltWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<salts::table>) -> Result<Vec<SaltWithId>, Error> {
        Ok(salts::table
            .filter(by)
            .get_results::<SaltWithId>(&connection())?)
    }

    fn update(&self, model: &Salt, by: Expr<salts::table>) -> Result<SaltWithId, Error> {
        Ok(update(salts::table)
            .filter(by)
            .set(model)
            .get_result::<SaltWithId>(&connection())?)
    }
}
