use crate::{
    account::{change_password::ChangePasswordPayload, Account, AccountController},
    policy::Bearer,
    result::WebResult,
    schema,
    validate::Validator,
};

use compat_uuid::Uuid;
use diesel::ExpressionMethods;
use postgres_resource::ResourceController;
use rocket::{get, http::Status, post, put, response::status::Custom};
use rocket_contrib::{json::Json, uuid::Uuid as rocketUuid};

#[get("/accounts/<id>", format = "application/json")]
pub fn get_account(_policy: Bearer, id: rocketUuid) -> WebResult {
    let account = AccountController
        .get_one(Box::new(schema::accounts::uuid.eq(Uuid::from(id))))
        .map_err(|e| match e {
            _ => err!(Status::Unauthorized, "account not found"),
        })?
        .inner;

    Ok(json!({ "model": account }))
}

#[post("/accounts", format = "application/json", data = "<account>")]
pub fn create_account(_policy: Bearer, account: Json<Account>) -> WebResult {
    let mut model = account.into_inner();
    let _ = model.validate()?;

    model.uuid = Some(Uuid::new());
    model.hash_password();

    let account = AccountController
        .create(&model)
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to create account"),
        })?
        .inner;

    Ok(json!({ "model": account }))
}

#[put("/accounts/<id>", format = "application/json", data = "<payload>")]
pub fn update_account(_policy: Bearer, id: rocketUuid, payload: Json<Account>) -> WebResult {
    let mut model = payload.into_inner();
    model.uuid = None;

    let account = AccountController
        .update(&model, Box::new(schema::accounts::uuid.eq(Uuid::from(id))))
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to update account"),
        })?
        .inner;

    Ok(json!({ "model": account }))
}

#[post("/accounts/password", format = "application/json", data = "<payload>")]
pub fn change_password(_policy: Bearer, payload: Json<ChangePasswordPayload>) -> WebResult {
    AccountController.change_password(payload.into_inner())?;
    Ok(json!({}))
}
