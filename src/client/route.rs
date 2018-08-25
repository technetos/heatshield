use client::{
    controller::ClientController,
    model::{Client, ClientWithId},
};
use controller::ResourceController;
use diesel::ExpressionMethods;
use jsonwebtoken;
use policy::Bearer;
use rocket_contrib::{Json, Value, UUID};
use schema;
use std::error::Error;
use uuid::Uuid;
use validate::Validator;

#[get("/clients/<id>", format = "application/json")]
pub fn get_client(_policy: Bearer, id: UUID) -> Result<Json, Json> {
    match ClientController.get_one(Box::new(schema::clients::uuid.eq(id.into_inner()))) {
        Ok(model) => Ok(Json(json!({ "model": model.client }))),
        Err(e) => Err(Json(
            json!({ "message": "get failed", "error": e.description() }),
        )),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateClientPayload {
    name: String,
    email: String,
}

#[post("/clients", format = "application/json", data = "<payload>")]
pub fn create_client(_policy: Bearer, payload: Json<CreateClientPayload>) -> Result<Json, Json> {
    let mut payload = payload.into_inner();

    let client = Client {
        email: Some(payload.email),
        name: Some(payload.name),
        uuid: Uuid::new_v4(),
    };

    client.validate()?;

    match ClientController.create(&client) {
        Ok(model) => Ok(Json(json!({ "model": model.client }))),
        Err(e) => Err(Json(json!("create failed"))),
    }
}