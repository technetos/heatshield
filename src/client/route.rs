use crate::{
    client::{Client, ClientController, ClientWithId},
    policy::Bearer,
    result::WebResult,
    schema,
    validate::Validator,
};

use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::{Json, Value, UUID};
use std::error::Error;
use uuid::Uuid;

#[get("/clients/<id>", format = "application/json")]
pub fn get_client(_policy: Bearer, id: UUID) -> Result<Json, Custom<Json>> {
    let client = ClientController
        .get_one(Box::new(schema::clients::uuid.eq(id.into_inner())))
        .map_err(|e| match e {
            _ => err!(Status::BadRequest, "no client found"),
        })?
        .client;

    Ok(Json(json!({ "model": client })))
}

#[derive(Serialize, Deserialize)]
pub struct CreateClientPayload {
    name: String,
    email: String,
}

#[post("/clients", format = "application/json", data = "<payload>")]
pub fn create_client(_policy: Bearer, payload: Json<CreateClientPayload>) -> WebResult {
    let payload = payload.into_inner();

    let client =
        Client { email: Some(payload.email), name: Some(payload.name), uuid: Uuid::new_v4() };

    client.validate()?;

    let client = ClientController
        .create(&client)
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to create client"),
        })?
        .client;

    Ok(Json(json!({ "model": client })))
}
