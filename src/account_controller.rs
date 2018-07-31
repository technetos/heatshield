use controller::ResourceController;
use diesel::{
    expression::BoxableExpression, insert_into, pg::Pg, prelude::*, result::Error, sql_types::Bool,
    update,
};
use failure;
use model::{Account, AccountWithId};
use rocket_contrib::Json;
use router::Action;
use schema::accounts;

pub struct AccountController;

impl<'a> Action<'a, Account, AccountWithId, accounts::table, accounts::SqlType>
    for AccountController
{
    fn create(&mut self, model: Json<Account>) -> Result<Json<AccountWithId>, failure::Error> {
        let account_model: Account = model.into_inner();

        match self._create(&account_model) {
            Ok(model) => Ok(Json(model)),
            Err(e) => panic!(),
        }
    }

    fn get_one(&mut self, id: i32) -> Result<Json<AccountWithId>, failure::Error> {
        match self._get_one(&|| Box::new(accounts::id.eq(id))) {
            Ok(model) => Ok(Json(model)),
            Err(e) => panic!(),
        }
    }

    fn update(
        &mut self,
        model: Json<AccountWithId>,
    ) -> Result<Json<AccountWithId>, failure::Error> {
        let account_model: AccountWithId = model.into_inner();

        match self._update(&account_model.account, &|| {
            Box::new(accounts::id.eq(account_model.id))
        }) {
            Ok(model) => Ok(Json(model)),
            Err(e) => panic!(),
        }
    }

    //    fn delete(&mut self, model: Json<ModelWithId>) -> bool {}
}

use db::establish_connection as connection;

type Expr = Box<BoxableExpression<accounts::table, Pg, SqlType = Bool>>;

impl ResourceController<Account, AccountWithId, accounts::table, accounts::SqlType>
    for AccountController
{
    fn _create(&self, model: &Account) -> Result<AccountWithId, Error> {
        Ok(insert_into(accounts::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn _get_one(&self, by: &Fn() -> Expr) -> Result<AccountWithId, Error> {
        Ok(accounts::table
            .filter(by())
            .get_result::<AccountWithId>(&connection())?)
    }

    fn _get_all(&self, by: &Fn() -> Expr) -> Result<Vec<AccountWithId>, Error> {
        Ok(accounts::table
            .filter(by())
            .get_results::<AccountWithId>(&connection())?)
    }

    fn _update(&self, model: &Account, by: &Fn() -> Expr) -> Result<AccountWithId, Error> {
        Ok(update(accounts::table)
            .filter(by())
            .set(model)
            .get_result::<AccountWithId>(&connection())?)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {}
}
