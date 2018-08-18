use client::controller::ClientController;
use controller::ResourceController;
use diesel::ExpressionMethods;
use policy::Bearer;
use rocket_contrib::{Json, Value, UUID};
use schema;
use std::error::Error;
use token::{
    controller::AccessTokenController,
    model::{AccessToken, AccessTokenWithId},
};
use validate::Validator;

use jsonwebtoken as jwt;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    client_id: Uuid,
    client_secret: Uuid,
    grant_type: String,
}

impl Validator for TokenPayload {
    fn validate(&self) -> Result<(), Json> {
        if self.grant_type != "client_credentials" {
            return Err(Json(json!("invalid grant_type")));
        }

        Ok(())
    }
}

#[post("/token", format = "application/json", data = "<payload>")]
pub fn get_token(payload: Json<TokenPayload>) -> Result<Json, Json> {
    let payload = payload.into_inner();
    payload.validate()?;

    match ClientController.get_one(Box::new(schema::clients::uuid.eq(payload.client_id))) {
        Err(e) => {
            return Err(Json(json!("no client found")));
        }
        Ok(model) => {
            // We have a valid client, create an access token for the client and
            // write it to the database
            let access_token = AccessToken {
                client_id: model.client.uuid,
            };
            match AccessTokenController.create(&access_token) {
                Err(e) => {
                    return Err(Json(json!("error creating token")));
                }
                Ok(tok) => {
                    println!("{}", &model.client.uuid);
                    // The token was created successfully, encode it as jwt
                    let encoded_token = jwt::encode(
                        &jwt::Header::default(),
                        &tok.access_token,
                        payload.client_secret.as_bytes(),
                    );

                    if let Ok(token_payload) = encoded_token {
                      return Ok(Json(json!(format!("Bearer {}", token_payload))));
                    } else {
                      return Err(Json(json!("error generating token")));
                    }
                }
            }
        }
    }
}
