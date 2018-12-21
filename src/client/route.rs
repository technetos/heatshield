use crate::{
    client::{Client, ClientController},
    policy::Bearer,
    result::WebResult,
    schema,
    validate::Validator,
};

use compat_uuid::Uuid;
use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{get, http::Status, post, response::status::Custom};
use rocket_contrib::{json::Json, uuid::Uuid as rocketUuid};

#[get("/clients/<id>", format = "application/json")]
pub fn get_client(_policy: Bearer, id: rocketUuid) -> WebResult {
    let client = ClientController
        .get_one(Box::new(schema::clients::uuid.eq(Uuid::from(id))))
        .map_err(|e| match e {
            _ => err!(Status::BadRequest, "no client found"),
        })?
        .inner;

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

    let client = Client {
        email: Some(payload.email),
        name: Some(payload.name),
        uuid: Uuid::new(),
    };

    client.validate()?;

    let client = ClientController
        .create(&client)
        .map_err(|e| match e {
            _ => err!(Status::InternalServerError, "unable to create client"),
        })?
        .inner;

    Ok(json!({ "model": client }))
}
