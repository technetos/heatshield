use crate::{
    account::controller::AccountController,
    refresh_token::{
        controller::RefreshTokenController,
        model::{RefreshToken, RefreshTokenWithId},
    },
    schema,
    token::LoginPayload,
    user_token::{
        controller::UserTokenController,
        model::{UserToken, UserTokenWithId},
    },
};

use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::{self, controller::*};
use rocket::Response;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::{Json, Value, UUID};
use uuid::Uuid;

pub trait Granter {
    fn grant_token(self) -> Result<Json, Custom<Json>>;
}

pub fn grant_token<T>(granter: T) -> Result<Json, Custom<Json>>
where
    T: Granter,
{
    granter.grant_token()
}

pub struct Password {
    client_id: Uuid,
    credentials: LoginPayload,
}

impl Password {
    pub fn new(client_id: Uuid, credentials: LoginPayload) -> Self {
        Self {
            client_id,
            credentials,
        }
    }
}

impl<'a> Granter for Password {
    fn grant_token(self) -> Result<Json, Custom<Json>> {
        let account = AccountController
            .get_one(Box::new(
                schema::accounts::username.eq(self.credentials.username),
            ))
            .map_err(|e| match e {
                _ => Custom(Status::Unauthorized, Json(json!("invalid credentials"))),
            })?
            .account;

        if !account.verify_password(&self.credentials.password) {
            return Err(Custom(
                Status::Unauthorized,
                Json(json!("invalid credentials")),
            ));
        }

        let refresh_token = RefreshTokenController
            .create(&RefreshToken {
                uuid: Uuid::new_v4(),
            })
            .map_err(|e| match e {
                _ => Custom(
                    Status::InternalServerError,
                    Json(json!("unable to create refresh token")),
                ),
            })?
            .refresh_token;

        let token = UserTokenController
            .create(&UserToken {
                client_id: self.client_id,
                account_id: account.uuid.unwrap(),
                refresh_id: Some(refresh_token.uuid),
            })
            .map_err(|e| match e {
                _ => Custom(
                    Status::InternalServerError,
                    Json(json!("unable to create user token")),
                ),
            })?
            .user_token;

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &token, b"secret")
            .map_err(|e| match e {
                _ => Custom(
                    Status::InternalServerError,
                    Json(json!("error creating jsonwebtoken")),
                ),
            })?;

        Ok(Json(json!(format!("Bearer {}", jwt))))
    }
}

pub struct Refresh {
    client_id: Uuid,
    refresh_id: Uuid,
    account_id: Uuid,
}

impl Refresh {
    pub fn new(client_id: Uuid, refresh_id: Uuid, account_id: Uuid) -> Self {
        Self {
            client_id,
            refresh_id,
            account_id,
        }
    }
}

impl Granter for Refresh {
    fn grant_token(self) -> Result<Json, Custom<Json>> {
        let refresh_token = RefreshTokenController
            .get_one(Box::new(schema::refresh_tokens::uuid.eq(self.refresh_id)))
            .map_err(|e| match e {
                _ => Custom(Status::BadRequest, Json(json!("invalid refresh_id"))),
            })?
            .refresh_token;

        let account = AccountController
            .get_one(Box::new(schema::accounts::uuid.eq(self.account_id)))
            .map_err(|e| match e {
                _ => Custom(Status::BadRequest, Json(json!("invalid account"))),
            })?
            .account;

        let token = UserTokenController
            .create(&UserToken {
                client_id: self.client_id,
                account_id: account.uuid.unwrap(),
                refresh_id: Some(refresh_token.uuid),
            })
            .map_err(|e| match e {
                _ => Custom(
                    Status::InternalServerError,
                    Json(json!("unable to create user token")),
                ),
            })?
            .user_token;

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &token, b"secret")
            .map_err(|e| match e {
                _ => Custom(
                    Status::InternalServerError,
                    Json(json!("error creating jsonwebtoken")),
                ),
            })?;

        Ok(Json(json!(format!("Bearer {}", jwt))))
    }
}
