use crate::{
    account::AccountController,
    refresh_token::{RefreshToken, RefreshTokenController, RefreshTokenWithId},
    result::WebResult,
    schema,
    token::LoginPayload,
    user_token::{UserToken, UserTokenController, UserTokenWithId},
};

use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{http::Status, response::status::Custom, Response};
use rocket_contrib::{uuid::Uuid as rocketUuid, json::JsonValue};
use compat_uuid::Uuid;

pub trait Granter {
    fn grant_token(self) -> WebResult;
}

pub fn grant_token<T>(granter: T) -> WebResult
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
        Self { client_id, credentials }
    }
}

impl<'a> Granter for Password {
    fn grant_token(self) -> WebResult {
        let account = AccountController
            .get_one(Box::new(schema::accounts::username.eq(self.credentials.username)))
            .map_err(|e| match e {
                _ => err!(Status::Unauthorized, "invalid credentials"),
            })?
            .account;

        if !account.verify_password(&self.credentials.password) {
            return Err(err!(Status::Unauthorized, "invalid credentials"));
        }

        let refresh_token = RefreshTokenController
            .create(&RefreshToken { uuid: Uuid::from(Uuid::new()) })
            .map_err(|e| match e {
                _ => err!(Status::InternalServerError, "error creating refresh token"),
            })?
            .refresh_token;

        let token = UserTokenController
            .create(&UserToken {
                client_id: self.client_id,
                account_id: account.uuid.unwrap(),
                refresh_id: Some(refresh_token.uuid),
            })
            .map_err(|e| match e {
                _ => err!(Status::InternalServerError, "error creating user token"),
            })?
            .user_token;

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &token, b"secret")
            .map_err(|e| match e {
                _ => err!(Status::InternalServerError, "error creating jsonwebtoken"),
            })?;

        Ok(json!(format!("Bearer {}", jwt)))
    }
}

pub struct Refresh {
    client_id: Uuid,
    refresh_id: Uuid,
    account_id: Uuid,
}

impl Refresh {
    pub fn new(client_id: Uuid, refresh_id: Uuid, account_id: Uuid) -> Self {
        Self { client_id, refresh_id, account_id }
    }
}

impl Granter for Refresh {
    fn grant_token(self) -> WebResult {
        let refresh_token = RefreshTokenController
            .get_one(Box::new(schema::refresh_tokens::uuid.eq(self.refresh_id)))
            .map_err(|e| match e {
                _ => err!(Status::BadRequest, "invalid refresh_id"),
            })?
            .refresh_token;

        let account = AccountController
            .get_one(Box::new(schema::accounts::uuid.eq(self.account_id)))
            .map_err(|e| match e {
                _ => err!(Status::BadRequest, "invalid account"),
            })?
            .account;

        let token = UserTokenController
            .create(&UserToken {
                client_id: self.client_id,
                account_id: account.uuid.unwrap(),
                refresh_id: Some(refresh_token.uuid),
            })
            .map_err(|e| match e {
                _ => err!(Status::InternalServerError, "unable to create user token"),
            })?
            .user_token;

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &token, b"secret")
            .map_err(|e| match e {
                _ => err!(Status::InternalServerError, "error creating jsonwebtoken"),
            })?;

        Ok(json!(format!("Bearer {}", jwt)))
    }
}
