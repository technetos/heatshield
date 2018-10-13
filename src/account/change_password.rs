use crate::{
    account::{controller::AccountController, model::Account},
    controller::ResourceController,
    sanitize::Sanitizer,
    schema,
    validate::Validator,
};

use diesel::prelude::*;
use rocket_contrib::{Json, Value};
use std::error::Error;
use uuid::Uuid;

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
    fn validate(&self) -> Result<(), Json> {
        if self.password.current == self.password.new {
            Err(Json(json!(
                "current_password and new_password must not be the same"
            )))
        } else {
            Ok(())
        }
    }
}

impl AccountController {
    pub fn change_password(&self, payload: ChangePasswordPayload) -> Result<Json, Json> {
        let _ = payload.validate()?;

        let mut model = self
            .get_one(Box::new(schema::accounts::uuid.eq(payload.account_id)))
            .map_err(|_| Json(json!("account not found")))?;

        let mut account = &mut model.account;

        if !account.verify_password(&payload.password.current) {
            Err(Json(json!("invalid current_password")))
        } else {
            account.password = Some(payload.password.current);

            account.hash_password();

            let _ = self
                .update(&account, Box::new(schema::accounts::id.eq(model.id)))
                .map_err(|e| match e {
                    _ => Json(json!("unable to update account")),
                })?;

            Ok(Json(json!(true)))
        }
    }
}
