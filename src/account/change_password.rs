use crate::{account::AccountController, result::WebResult, schema, validate::Validator};

use compat_uuid::Uuid;
use diesel::prelude::*;
use postgres_resource::ResourceController;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangePasswordPayload {
    account_id: Uuid,
    password: Password,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    current: String,
    new: String,
}

impl Validator for ChangePasswordPayload {
    fn validate(&self) -> Result<(), Custom<JsonValue>> {
        if self.password.current == self.password.new {
            Err(err!(
                Status::BadRequest,
                "current_password and new_password must not be the same"
            ))
        } else {
            Ok(())
        }
    }
}

impl AccountController {
    pub fn change_password(&self, payload: ChangePasswordPayload) -> WebResult {
        let _ = payload.validate()?;

        let mut model = self
            .get_one(Box::new(schema::accounts::uuid.eq(payload.account_id)))
            .map_err(|_| err!(Status::BadRequest, "account not found"))?;

        let account = &mut model.inner;

        if !account.verify_password(&payload.password.current) {
            Err(err!(Status::Unauthorized, "invalid current_password"))
        } else {
            account.password = Some(payload.password.current);

            account.hash_password();

            let _ = self
                .update(&account, Box::new(schema::accounts::id.eq(model.id)))
                .map_err(|e| match e {
                    _ => err!(Status::InternalServerError, "unable to update account"),
                })?;

            Ok(json!(true))
        }
    }
}
