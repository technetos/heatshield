use controller::{Expr, ResourceController, ResourceTable, ResourceSql};
use diesel::{insert_into, prelude::*, result::Error, update};
use failure;
use model::{Account, AccountWithId};
use rocket_contrib::Json;
use router::Action;
use schema::accounts;

#[get("/accounts/<id>")]
pub fn account(id: usize) -> Json<AccountWithId> {
    AccountController.get_one(id as i32).unwrap()
}

pub struct AccountController;

impl Action for AccountController {
    fn create(&self, json: Json<Account>) -> Result<Json<AccountWithId>, failure::Error> {
        let model: Account = json.into_inner();

        match self._create(&model) {
            Ok(model) => Ok(Json(model)),
            Err(e) => panic!(),
        }
    }

    fn get_one(&self, id: i32) -> Result<Json<AccountWithId>, failure::Error> {
        match self._get_one(Box::new(accounts::id.eq(id))) {
            Ok(model) => Ok(Json(model)),
            Err(e) => panic!(),
        }
    }

    fn update(&self, json: Json<AccountWithId>) -> Result<Json<AccountWithId>, failure::Error> {
        let model: AccountWithId = json.into_inner();

        match self._update(&model.account, Box::new(accounts::id.eq(model.id))) {
            Ok(model) => Ok(Json(model)),
            Err(e) => panic!(e),
        }
    }

    //    fn delete(&mut self, model: Json<ModelWithId>) -> bool {}
}

impl ResourceTable for AccountController {
  type DBTable = accounts::table;
}

impl ResourceSql for AccountController {
  type SQLType = accounts::SqlType;
}

use db::establish_connection as connection;

impl ResourceController for AccountController {
    type Model = Account;
    type ModelWithId = AccountWithId;

    fn _create(&self, model: &Account) -> Result<AccountWithId, Error> {
        Ok(insert_into(accounts::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn _get_one(&self, by: Expr<accounts::table>) -> Result<AccountWithId, Error> {
        Ok(accounts::table
            .filter(by)
            .get_result::<AccountWithId>(&connection())?)
    }

    fn _get_all(&self, by: Expr<accounts::table>) -> Result<Vec<AccountWithId>, Error> {
        Ok(accounts::table
            .filter(by)
            .get_results::<AccountWithId>(&connection())?)
    }

    fn _update(&self, model: &Account, by: Expr<accounts::table>) -> Result<AccountWithId, Error> {
        Ok(update(accounts::table)
            .filter(by)
            .set(model)
            .get_result::<AccountWithId>(&connection())?)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
      AccountController._create(&Account {
        username: Some("bob".to_string()),
        password: Some("pass".to_string()),
        email: Some("email@domain.ext".to_string()),
        enabled: None,
      });
    }
}
