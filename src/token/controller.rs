use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use schema::access_tokens;
use token::model::{AccessToken, AccessTokenWithId};

pub struct AccessTokenController;

impl ResourceWithId for AccessTokenController {
    type ModelWithId = AccessTokenWithId;
}

impl Resource for AccessTokenController {
    type Model = AccessToken;
}

impl ResourceTable for AccessTokenController {
    type DBTable = access_tokens::table;
}

impl ResourceSql for AccessTokenController {
    type SQLType = access_tokens::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for AccessTokenController {
    fn create(&self, model: &AccessToken) -> Result<AccessTokenWithId, Error> {
        Ok(insert_into(access_tokens::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<access_tokens::table>) -> Result<AccessTokenWithId, Error> {
        Ok(access_tokens::table
            .filter(by)
            .get_result::<AccessTokenWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<access_tokens::table>) -> Result<Vec<AccessTokenWithId>, Error> {
        Ok(access_tokens::table
            .filter(by)
            .get_results::<AccessTokenWithId>(&connection())?)
    }

    fn update(
        &self,
        model: &AccessToken,
        by: Expr<access_tokens::table>,
    ) -> Result<AccessTokenWithId, Error> {
        Ok(update(access_tokens::table)
            .filter(by)
            .set(model)
            .get_result::<AccessTokenWithId>(&connection())?)
    }
}
