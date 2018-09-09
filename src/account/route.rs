use account::controller::AccountController;
use account::{
    change_password::ChangePasswordPayload,
    model::{Account, AccountWithId},
};
use controller::ResourceController;
use diesel::ExpressionMethods;
use policy::Bearer;
use rocket_contrib::{Json, Value, UUID};
use schema;
use std::error::Error;
use uuid::Uuid;
use validate::Validator;

#[get("/accounts/<id>", format = "application/json")]
pub fn get_account(_policy: Bearer, id: UUID) -> Result<Json<AccountWithId>, Json> {
    match AccountController.get_one(Box::new(schema::accounts::uuid.eq(id.into_inner()))) {
        Ok(model) => Ok(Json(model)),
        Err(e) => Err(Json(
            json!({ "message": "get failed", "error": e.description() }),
        )),
    }
}

#[post("/accounts", format = "application/json", data = "<account>")]
pub fn create_account(_policy: Bearer, account: Json<Account>) -> Result<Json, Json> {
    let mut model = account.into_inner();

    // Ensure the required fields are met
    model.validate()?;

    // Dont allow the uuid to be set manually
    model.uuid = Some(Uuid::new_v4());

    model.hash_password();

    match AccountController.create(&model) {
        Ok(model) => Ok(Json(json!({ "model": model }))),
        Err(e) => Err(Json(json!("create failed"))),
    }
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

    match AccountController.update(&model, Box::new(schema::accounts::uuid.eq(id.into_inner()))) {
        Ok(model) => Ok(Json(json!({ "model": model }))),
        Err(e) => Err(Json(json!("update failed"))),
    }
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
