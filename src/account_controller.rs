use controller::{ResourceController, ControllerLifecycle};
use diesel::{self, insert_into, prelude::*};
use model::{Account, AccountWithId};
use schema::accounts;

pub struct AccountController {
    model: AccountWithId,
}

impl ControllerLifecycle for AccountController {
    fn before_create(&mut self) {

    }

    fn create(&mut self) {
        // define account controller specific behavior for creating an account model
        // then call create_resource to actually create the model
        let Account {
            ref username,
            ref password,
            ref email,
            ..
        } = self.model.account;

        match self.create_resource(&self.model.account) {
            Ok(model) => {}
            Err(e) => {}
        }
    }

    fn after_create(&mut self) {

    }
}

impl AccountController {
    fn restore_if_deleted(&self) {}
}

use db::establish_connection as connection;

impl ResourceController<Account, AccountWithId, accounts::table, accounts::SqlType>
    for AccountController
{
    fn create_resource(&self, model: &Account) -> Result<AccountWithId, diesel::result::Error> {
        use schema::accounts::dsl::*;
        Ok(insert_into(accounts)
            .values(model)
            .get_result(&connection())?)
    }

    fn select_resource(&self, model: &AccountWithId) -> Result<(), diesel::result::Error> {
        Ok(())
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
                    username: None,
                    password: None,
                    email: None,
                    enabled: None,
                },
                verification_id: None,
            },
        };

        account_controller.create();
    }
}
