use account::model::{Account, AccountWithId};
use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use rocket_contrib::{Json, Value};
use schema::accounts;
use verification::model::{Confirmation, Verification};

pub struct AccountController;

impl ResourceWithId for AccountController {
    type ModelWithId = AccountWithId;
}

impl Resource for AccountController {
    type Model = Account;
}

impl ResourceTable for AccountController {
    type DBTable = accounts::table;
}

impl ResourceSql for AccountController {
    type SQLType = accounts::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for AccountController {
    fn create(&self, model: &Account) -> Result<AccountWithId, Error> {
        Ok(insert_into(accounts::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn get_one(&self, by: Expr<accounts::table>) -> Result<AccountWithId, Error> {
        Ok(accounts::table
            .filter(by)
            .get_result::<AccountWithId>(&connection())?)
    }

    fn get_all(&self, by: Expr<accounts::table>) -> Result<Vec<AccountWithId>, Error> {
        Ok(accounts::table
            .filter(by)
            .get_results::<AccountWithId>(&connection())?)
    }

    fn update(&self, model: &Account, by: Expr<accounts::table>) -> Result<AccountWithId, Error> {
        Ok(update(accounts::table)
            .filter(by)
            .set(model)
            .get_result::<AccountWithId>(&connection())?)
    }
}
