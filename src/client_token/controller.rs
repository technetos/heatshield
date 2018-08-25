use client_token::model::{ClientToken, ClientTokenWithId};
use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use schema::client_tokens;

pub struct ClientTokenController;

impl ResourceWithId for ClientTokenController {
    type ModelWithId = ClientTokenWithId;
}

impl Resource for ClientTokenController {
    type Model = ClientToken;
}

impl ResourceTable for ClientTokenController {
    type DBTable = client_tokens::table;
}

impl ResourceSql for ClientTokenController {
    type SQLType = client_tokens::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for ClientTokenController {
    fn create(&self, model: &ClientToken) -> Result<ClientTokenWithId, Error> {
        Ok(insert_into(client_tokens::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<client_tokens::table>) -> Result<ClientTokenWithId, Error> {
        Ok(client_tokens::table
            .filter(by)
            .get_result::<ClientTokenWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<client_tokens::table>) -> Result<Vec<ClientTokenWithId>, Error> {
        Ok(client_tokens::table
            .filter(by)
            .get_results::<ClientTokenWithId>(&connection())?)
    }

    fn update(
        &self,
        model: &ClientToken,
        by: Expr<client_tokens::table>,
    ) -> Result<ClientTokenWithId, Error> {
        Ok(update(client_tokens::table)
            .filter(by)
            .set(model)
            .get_result::<ClientTokenWithId>(&connection())?)
    }
}
