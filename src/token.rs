use account::controller::AccountController;
use client::controller::ClientController;
use client_token::{
    controller::ClientTokenController,
    model::{ClientToken, ClientTokenWithId},
};
use controller::ResourceController;
use diesel::ExpressionMethods;
use granter::Granter;
use policy::Bearer;
use rocket_contrib::{Json, Value, UUID};
use schema;
use std::error::Error;
use user_token::{
    controller::UserTokenController,
    model::{UserToken, UserTokenWithId},
};
use validate::Validator;

use jsonwebtoken;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    client_id: Option<Uuid>,
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

        if self.credentials.is_none() {
            return Err(Json(json!("credentials required")));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[post("/token", format = "application/json", data = "<payload>")]
pub fn get_token(payload: Json<TokenPayload>) -> Result<Json, Json> {
    let payload = payload.into_inner();

    let _ = payload.validate()?;

    let granter = Granter::from_str(payload.grant_type.unwrap())?;

    let client = ClientController
        .get_one(Box::new(
            schema::clients::uuid.eq(payload.client_id.unwrap()),
        )).map_err(|_| Json(json!("client not found")))?
        .client;

    match granter {
        Granter::Password => {
            let credentials = payload.credentials.unwrap();
            let account = AccountController
                .get_one(Box::new(
                    schema::accounts::username.eq(credentials.username),
                )).map_err(|_| Json(json!("invalid credentials")))?
                .account;

            if !account.verify_password(&credentials.password) {
                return Err(Json(json!("invalid credentials")));
            }

            let token = UserTokenController
                .create(&UserToken {
                    client_id: client.uuid,
                    account_id: account.uuid.unwrap(),
                    refresh_id: Uuid::new_v4(),
                }).map_err(|_| Json(json!("unable to create user token")))?
                .user_token;

            let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &token, b"secret")
                .map_err(|_| Json(json!("error creating jsonwebtoken")))?;

            Ok(Json(json!(format!("Bearer {}", jwt))))
        }
    }
}
