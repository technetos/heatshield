use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use client::model::{Client, ClientWithId};
use schema::clients;

pub struct ClientController;

impl ResourceWithId for ClientController {
    type ModelWithId = ClientWithId;
}

impl Resource for ClientController {
    type Model = Client;
}

impl ResourceTable for ClientController {
    type DBTable = clients::table;
}

impl ResourceSql for ClientController {
    type SQLType = clients::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for ClientController {
    fn create(&self, model: &Client) -> Result<ClientWithId, Error> {
        Ok(insert_into(clients::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<clients::table>) -> Result<ClientWithId, Error> {
        Ok(clients::table
            .filter(by)
            .get_result::<ClientWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<clients::table>) -> Result<Vec<ClientWithId>, Error> {
        Ok(clients::table
            .filter(by)
            .get_results::<ClientWithId>(&connection())?)
    }

    fn update(&self, model: &Client, by: Expr<clients::table>) -> Result<ClientWithId, Error> {
        Ok(update(clients::table)
            .filter(by)
            .set(model)
            .get_result::<ClientWithId>(&connection())?)
    }
}
