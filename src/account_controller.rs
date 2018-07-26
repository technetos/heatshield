use controller::{ControllerLifecycle, ResourceController};
use diesel::{
    self, expression::BoxableExpression, insert_into, pg::Pg, prelude::*, result::Error,
    sql_types::Bool,
};
use model::{Account, AccountWithId};
use schema::accounts;

pub struct AccountController {
    model: AccountWithId,
}

impl ControllerLifecycle for AccountController {
    fn before_create(&mut self) {
        let Account {
            ref username,
            ref password,
            ref email,
            ..
        } = self.model.account;
    }

    fn create(&mut self) {
        // define account controller specific behavior for creating an account model
        // then call _create to actually create the model
        match self._create(&self.model.account) {
            Ok(model) => self.model = model,
            Err(e) => {}
        }
    }

    fn get_one(&mut self, id: i32) {
        self.model.id = id;

        match self._get_one(&|account| Box::new(accounts::id.eq(account.id))) {
            Ok(model) => self.model = model,
            Err(e) => {}
        }
    }
}

impl AccountController {
    fn restore_if_deleted(&self) {}
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

    fn _get_one(&self, by: &Fn(&AccountWithId) -> Expr) -> Result<AccountWithId, Error> {
        Ok(accounts::table
            .filter(by(&self.model))
            .get_result::<AccountWithId>(&connection())?)
    }

    fn _get_all(&self, by: &Fn(&AccountWithId) -> Expr) -> Result<Vec<AccountWithId>, Error> {
        Ok(accounts::table
            .filter(by(&self.model))
            .get_results::<AccountWithId>(&connection())?)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut account_controller = AccountController {
            model: AccountWithId {
                id: 0,
                account: Account {
                    username: Some("foo".to_owned()),
                    password: Some("foo".to_owned()),
                    email: Some("foo".to_owned()),
                    enabled: None,
                },
                verification_id: None,
            },
        };

        account_controller.create();
        account_controller.get_one(1);
    }
}
