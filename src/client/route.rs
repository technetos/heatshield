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
use rocket::{http::Status, response::status::Custom, get, post};
use rocket_contrib::{uuid::Uuid as rocketUuid, json::{Json, JsonValue}};
use compat_uuid::Uuid;
use std::error::Error;

#[get("/clients/<id>", format = "application/json")]
pub fn get_client(_policy: Bearer, id: rocketUuid) -> WebResult {
    let client = ClientController
        .get_one(Box::new(schema::clients::uuid.eq(Uuid::from(id))))
        .map_err(|e| match e {
            _ => err!(Status::BadRequest, "no client found"),
        })?
        .client;

    Ok(json!({ "model": client }))
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
        Client { email: Some(payload.email), name: Some(payload.name), uuid: Uuid::new() };

    client.validate()?;

    let client = ClientController
        .create(&client)
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to create client"),
        })?
        .client;

    Ok(json!({ "model": client }))
}
