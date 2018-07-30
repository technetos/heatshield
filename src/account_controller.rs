use controller::{ResourceController, ResourceControllerLifecycle};
use diesel::{
    self, expression::BoxableExpression, insert_into, pg::Pg, prelude::*, result::Error,
    sql_types::Bool,
};
use model::{Account, AccountWithId};
use rocket_contrib::Json;
use router::Action;
use schema::accounts;
use serde::{Deserialize, Serialize};
use serde_json;

pub struct AccountController {
    model: Option<AccountWithId>,
}

impl<'a> Action<'a, Account, AccountWithId> for AccountController {
    fn create(&mut self, model: Json<Account>) -> Json<AccountWithId> {
        self.reset();

        self.model.as_mut().unwrap().account = model.into_inner();
        self._create();
        Json(self.model.take().unwrap())
    }
    //  fn get_all(&mut self) -> Json<Vec<ModelWithId>>;
    fn get_one(&mut self, id: i32) -> Json<AccountWithId> {
        self.reset();

        self.model.as_mut().unwrap().id = id;
        self._get_one();
        Json(self.model.take().unwrap())
    }
    //  fn update(&mut self, model: Json<ModelWithId>) -> Json<ModelWithId>;
    //  fn delete(&mut self, model: Json<ModelWithId>) -> bool;
}

impl ResourceControllerLifecycle for AccountController {
    fn _create(&mut self) -> Result<(), ()> {
        let valid = self.validate();

        if valid {
            match self.__create(&self.model.as_ref().unwrap().account) {
                Ok(model) => {
                    self.model = Some(model);
                    Ok(())
                }
                Err(e) => Err(()),
            }
        } else {
            Err(())
        }
    }

    fn _get_one(&mut self) {
        match self.__get_one(&|account| Box::new(accounts::id.eq(account.id))) {
            Ok(model) => self.model = Some(model),
            Err(e) => {}
        }
    }
}

impl AccountController {
    fn reset(&mut self) {
        self.model = Some(AccountWithId {
            id: 0,
            account: Account {
                username: None,
                password: None,
                email: None,
                enabled: None,
            },
            verification_id: None,
        });
    }

    fn validate(&self) -> bool {
        let Account {
            ref username,
            ref password,
            ref email,
            ..
        } = self.model.as_ref().unwrap().account;

        username.is_none() || password.is_none() || email.is_none()
    }

    fn restore_if_deleted(&self) {}
}

use db::establish_connection as connection;

type Expr = Box<BoxableExpression<accounts::table, Pg, SqlType = Bool>>;

impl ResourceController<Account, AccountWithId, accounts::table, accounts::SqlType>
    for AccountController
{
    fn __create(&self, model: &Account) -> Result<AccountWithId, Error> {
        Ok(insert_into(accounts::table)
            .values(model)
            .get_result(&connection())?)
    }

    fn __get_one(&self, by: &Fn(&AccountWithId) -> Expr) -> Result<AccountWithId, Error> {
        Ok(accounts::table
            .filter(by(&self.model.as_ref().unwrap()))
            .get_result::<AccountWithId>(&connection())?)
    }

    fn __get_all(&self, by: &Fn(&AccountWithId) -> Expr) -> Result<Vec<AccountWithId>, Error> {
        Ok(accounts::table
            .filter(by(&self.model.as_ref().unwrap()))
            .get_results::<AccountWithId>(&connection())?)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut account_controller = AccountController { model: None };
        account_controller._create();
        //        account_controller.model.id = 1;
        //        account_controller.get_one();
        //        assert_eq!(account_controller.model.account.username, Some("bob".to_owned()))
    }
}
