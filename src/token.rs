use crate::{
    client::controller::ClientController,
    granter::{grant_token, Granter, Password, Refresh},
    schema,
    validate::Validator,
};

use postgres_resource::{self, controller::*};
use diesel::ExpressionMethods;
use jsonwebtoken;
use rocket_contrib::{Json, Value, UUID};
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    client_id: Option<Uuid>,
    refresh_id: Option<Uuid>,
    account_id: Option<Uuid>,
    grant_type: Option<String>,
    credentials: Option<LoginPayload>,
}

impl Validator for TokenPayload {
    fn validate(&self) -> Result<(), Json> {
        if self.client_id.is_none() {
            return Err(Json(json!("client_id required")));
        }

        if self.grant_type.is_none() {
            return Err(Json(json!("grant_type required")));
        }

        match &self.grant_type.as_ref().unwrap()[..] {
            "password" if self.credentials.is_none() => {
                return Err(Json(json!("credentials required")))
            }
            "refresh_token" if self.refresh_id.is_none() => {
                return Err(Json(json!("refresh_id required")))
            }
            "refresh_token" if self.account_id.is_none() => {
                return Err(Json(json!("account_id required")))
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
pub fn get_token(payload: Json<TokenPayload>) -> Result<Json, Json> {
    let mut payload = payload.into_inner();

    let _ = payload.validate()?;

    let client = ClientController
        .get_one(Box::new(
            schema::clients::uuid.eq(payload.client_id.unwrap()),
        ))
        .map_err(|_| Json(json!("invalid client")))?
        .client;

    match &payload.grant_type.unwrap()[..] {
        "password" => grant_token(Password::new(client.uuid, payload.credentials.unwrap())),
        "refresh_token" => grant_token(Refresh::new(
            client.uuid,
            payload.account_id.unwrap(),
            payload.refresh_id.unwrap(),
        )),
        _ => Err(Json(json!("invalid grant_type"))),
    }
}
