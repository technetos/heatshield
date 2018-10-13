use crate::{
    account::controller::AccountController,
    account::{
        change_password::ChangePasswordPayload,
        model::{Account, AccountWithId},
    },
    controller::ResourceController,
    policy::Bearer,
    schema,
    validate::Validator,
};

use diesel::ExpressionMethods;
use rocket_contrib::{Json, Value, UUID};
use std::error::Error;
use uuid::Uuid;

#[get("/accounts/<id>", format = "application/json")]
pub fn get_account(_policy: Bearer, id: UUID) -> Result<Json, Json> {
    let account = AccountController
        .get_one(Box::new(schema::accounts::uuid.eq(id.into_inner())))
        .map_err(|e| match e {
            _ => Json(json!("account not found")),
        })?
        .account;

    Ok(Json(json!({ "model": account })))
}

#[post("/accounts", format = "application/json", data = "<account>")]
pub fn create_account(_policy: Bearer, account: Json<Account>) -> Result<Json, Json> {
    let mut model = account.into_inner();

    let _ = model.validate()?;

    model.uuid = Some(Uuid::new_v4());

    model.hash_password();

    let account = AccountController
        .create(&model)
        .map_err(|e| match e {
            _ => Json(json!("unable to create account")),
        })?
        .account;

    Ok(Json(json!({ "model": account })))
}

#[put(
    "/accounts/<id>",
    format = "application/json",
    data = "<payload>"
)]
pub fn update_account(_policy: Bearer, id: UUID, payload: Json<Account>) -> Result<Json, Json> {
    let mut model = payload.into_inner();

    // Prevent the uuid from being changed manually
    model.uuid = None;

    let account = AccountController
        .update(&model, Box::new(schema::accounts::uuid.eq(id.into_inner())))
        .map_err(|e| match e {
            _ => Json(json!("unable to update account")),
        })?
        .account;

    Ok(Json(json!({ "model": account })))
}

#[post(
    "/accounts/password",
    format = "application/json",
    data = "<payload>"
)]
pub fn change_password(
    _policy: Bearer,
    payload: Json<ChangePasswordPayload>,
) -> Result<Json, Json> {
    AccountController.change_password(payload.into_inner())?;
    Ok(Json(json!({})))
}
