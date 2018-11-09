use crate::{
    account::{change_password::ChangePasswordPayload, Account, AccountController, AccountWithId},
    policy::Bearer,
    result::WebResult,
    schema,
    validate::Validator,
};

use diesel::ExpressionMethods;
use postgres_resource::ResourceController;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::{Json, Value, UUID};
use std::error::Error;
use uuid::Uuid;

#[get("/accounts/<id>", format = "application/json")]
pub fn get_account(_policy: Bearer, id: UUID) -> WebResult {
    let account = AccountController
        .get_one(Box::new(schema::accounts::uuid.eq(id.into_inner())))
        .map_err(|e| match e {
            _ => err!(Status::Unauthorized, "account not found"),
        })?
        .account;

    Ok(Json(json!({ "model": account })))
}

#[post("/accounts", format = "application/json", data = "<account>")]
pub fn create_account(_policy: Bearer, account: Json<Account>) -> WebResult {
    let mut model = account.into_inner();
    let _ = model.validate()?;

    model.uuid = Some(Uuid::new_v4());
    model.hash_password();

    let account = AccountController
        .create(&model)
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to create account"),
        })?
        .account;

    Ok(Json(json!({ "model": account })))
}

#[put("/accounts/<id>", format = "application/json", data = "<payload>")]
pub fn update_account(_policy: Bearer, id: UUID, payload: Json<Account>) -> WebResult {
    let mut model = payload.into_inner();
    model.uuid = None;

    let account = AccountController
        .update(&model, Box::new(schema::accounts::uuid.eq(id.into_inner())))
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to update account"),
        })?
        .account;

    Ok(Json(json!({ "model": account })))
}

#[post("/accounts/password", format = "application/json", data = "<payload>")]
pub fn change_password(_policy: Bearer, payload: Json<ChangePasswordPayload>) -> WebResult {
    AccountController.change_password(payload.into_inner())?;
    Ok(Json(json!({})))
}
