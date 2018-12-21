use crate::{
    client::ClientController,
    granter::{grant_token, Password, Refresh},
    result::WebResult,
    schema,
    validate::Validator,
};

use compat_uuid::Uuid;
use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{http::Status, post, response::status::Custom};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    client_id: Option<Uuid>,
    refresh_id: Option<Uuid>,
    grant_type: Option<String>,
    credentials: Option<LoginPayload>,
}

impl Validator for TokenPayload {
    fn validate(&self) -> Result<(), Custom<JsonValue>> {
        if self.client_id.is_none() {
            return Err(err!(Status::BadRequest, "client_id required"));
        }
        if self.grant_type.is_none() {
            return Err(err!(Status::BadRequest, "grant_type required"));
        }
        match &self.grant_type.as_ref().unwrap()[..] {
            "password" if self.credentials.is_none() => {
                return Err(err!(Status::BadRequest, "user credentials required"));
            }
            "refresh_token" if self.refresh_id.is_none() => {
                return Err(err!(Status::BadRequest, "refresh_id required"));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[post("/token", format = "application/json", data = "<payload>")]
pub fn get_token(payload: Json<TokenPayload>) -> WebResult {
    let payload = payload.into_inner();

    let _ = payload.validate()?;

    let client = ClientController
        .get_one(Box::new(
            schema::clients::uuid.eq(payload.client_id.unwrap()),
        ))
        .map_err(|_| err!(Status::BadRequest, "invalid client"))?
        .inner;

    match &payload.grant_type.unwrap()[..] {
        "password" => {
            let credentials = payload.credentials.unwrap();
            grant_token(Password::new(client.uuid, credentials))
        }
        "refresh_token" => {
            let refresh_id = payload.refresh_id.unwrap();
            grant_token(Refresh::new(refresh_id))
        }
        _ => Err(err!(Status::BadRequest, "invalid grant_type")),
    }
}
