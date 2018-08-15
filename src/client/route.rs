use client::{model::{ClientWithId, Client}, controller::ClientController};
use controller::ResourceController;
use diesel::ExpressionMethods;
use policy::Bearer;
use rocket_contrib::{Json, Value, UUID};
use schema;
use std::error::Error;
use uuid::Uuid;

#[get("/clients/<id>")]
pub fn get_account(_policy: Bearer, id: UUID) -> Result<Json, Json> {
    match ClientController.get_one(Box::new(schema::accounts::uuid.eq(id.into_inner()))) {
        Ok(model) => Ok(Json(model)),
        Err(e) => Err(Json(
            json!({ "message": "get failed", "error": e.description() }),
        )),
    }
}

#[post("/clients", format = "application/json", data = "<payload>")]
pub fn create_account(_policy: Bearer, account: Json<Client>) -> Result<Json, Json> {
    let mut model = account.into_inner();

    // Dont allow the uuid to be set manually
    model.uuid = Some(Uuid::new_v4());

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
