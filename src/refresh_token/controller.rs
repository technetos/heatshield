use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use refresh_token::model::{RefreshToken, RefreshTokenWithId};
use schema::refresh_tokens;

pub struct RefreshTokenController;

impl ResourceWithId for RefreshTokenController {
    type ModelWithId = RefreshTokenWithId;
}

impl Resource for RefreshTokenController {
    type Model = RefreshToken;
}

impl ResourceTable for RefreshTokenController {
    type DBTable = refresh_tokens::table;
}

impl ResourceSql for RefreshTokenController {
    type SQLType = refresh_tokens::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for RefreshTokenController {
    fn create(&self, model: &RefreshToken) -> Result<RefreshTokenWithId, Error> {
        Ok(insert_into(refresh_tokens::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<refresh_tokens::table>) -> Result<RefreshTokenWithId, Error> {
        Ok(refresh_tokens::table
            .filter(by)
            .get_result::<RefreshTokenWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<refresh_tokens::table>) -> Result<Vec<RefreshTokenWithId>, Error> {
        Ok(refresh_tokens::table
            .filter(by)
            .get_results::<RefreshTokenWithId>(&connection())?)
    }

    fn update(
        &self,
        model: &RefreshToken,
        by: Expr<refresh_tokens::table>,
    ) -> Result<RefreshTokenWithId, Error> {
        Ok(update(refresh_tokens::table)
            .filter(by)
            .set(model)
            .get_result::<RefreshTokenWithId>(&connection())?)
    }
}
