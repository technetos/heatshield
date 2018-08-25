use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use schema::user_tokens;
use user_token::model::{UserToken, UserTokenWithId};

pub struct UserTokenController;

impl ResourceWithId for UserTokenController {
    type ModelWithId = UserTokenWithId;
}

impl Resource for UserTokenController {
    type Model = UserToken;
}

impl ResourceTable for UserTokenController {
    type DBTable = user_tokens::table;
}

impl ResourceSql for UserTokenController {
    type SQLType = user_tokens::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for UserTokenController {
    fn create(&self, model: &UserToken) -> Result<UserTokenWithId, Error> {
        Ok(insert_into(user_tokens::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<user_tokens::table>) -> Result<UserTokenWithId, Error> {
        Ok(user_tokens::table
            .filter(by)
            .get_result::<UserTokenWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<user_tokens::table>) -> Result<Vec<UserTokenWithId>, Error> {
        Ok(user_tokens::table
            .filter(by)
            .get_results::<UserTokenWithId>(&connection())?)
    }

    fn update(
        &self,
        model: &UserToken,
        by: Expr<user_tokens::table>,
    ) -> Result<UserTokenWithId, Error> {
        Ok(update(user_tokens::table)
            .filter(by)
            .set(model)
            .get_result::<UserTokenWithId>(&connection())?)
    }
}
