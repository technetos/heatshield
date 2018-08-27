use account::{controller::AccountController, model::Account};
use controller::ResourceController;
use diesel::prelude::*;
use rocket_contrib::{Json, Value};
use sanitize::Sanitizer;
use std::error::Error;
use uuid::Uuid;
use validate::Validator;

use schema;

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
        payload.validate()?;

        let mut model = self
            .get_one(Box::new(schema::accounts::uuid.eq(payload.account_id)))
            .map_err(|_| Json(json!("account not found")))?;

        let mut account = &mut model.account;

        if account.verify_password(&payload.password.current) {
            account.password = Some(payload.password.current);

            let _ = self
                .update(&account, Box::new(schema::accounts::id.eq(model.id)))
                .map_err(|_| Json(json!("unable to update account")))?;

            Ok(Json(json!(true)))
        } else {
            Err(Json(json!("invalid current_password")))
        }
    }
}
