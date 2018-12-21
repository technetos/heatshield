use crate::{
    access_token::{AccessToken, AccessTokenController},
    account::AccountController,
    jwt::JWT,
    refresh_token::{RefreshToken, RefreshTokenController},
    result::WebResult,
    schema::{accounts, user_tokens},
    token::LoginPayload,
    user_token::{UserToken, UserTokenController},
};

use compat_uuid::Uuid;
use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{http::Status, response::status::Custom};

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
        Self {
            client_id,
            credentials,
        }
    }
}

impl<'a> Granter for Password {
    fn grant_token(self) -> WebResult {
        let account = AccountController
            .get_one(Box::new(accounts::username.eq(self.credentials.username)))
            .map_err(|_| err!(Status::Unauthorized, "invalid credentials"))?
            .inner;

        if !account.verify_password(&self.credentials.password) {
            return Err(err!(Status::Unauthorized, "invalid credentials"));
        }

        let refresh_token = RefreshTokenController
            .create(&RefreshToken {
                uuid: Uuid::from(Uuid::new()),
            })
            .map_err(|e| match e {
                _ => err!(Status::InternalServerError, "error creating refresh token"),
            })?
            .inner;

        let user_token = UserTokenController
            .create(&UserToken {
                client_id: self.client_id,
                account_id: account.uuid.unwrap(),
                refresh_id: Some(refresh_token.uuid),
            })
            .map_err(|_| err!(Status::InternalServerError, "error creating user token"))?;

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &JWT::new(), b"secret")
            .map_err(|_| err!(Status::InternalServerError, "error creating jsonwebtoken"))?;

        let access_token = AccessTokenController
            .create(&AccessToken {
                jwt,
                expires_in: 3600,
                user_id: user_token.id,
            })
            .map_err(|e| {
                println!("{}", e);
                err!(Status::InternalServerError, "error creating access token")
            })?;

        Ok(json!({
            "token_type": "Bearer",
            "access_token": &access_token.inner.jwt,
            "expires_in": &access_token.inner.expires_in,
            "refresh_token": &user_token.inner.refresh_id.unwrap(),
        }))
    }
}

pub struct Refresh {
    refresh_id: Uuid,
}

impl Refresh {
    pub fn new(refresh_id: Uuid) -> Self {
        Self { refresh_id }
    }
}

impl Granter for Refresh {
    fn grant_token(self) -> WebResult {
        let mut existing_user_token = UserTokenController
            .get_one(Box::new(user_tokens::refresh_id.eq(self.refresh_id)))
            .map_err(|_| err!(Status::Unauthorized, json!("Invalid refresh token")))?;

        let new_refresh_token = RefreshTokenController
            .create(&RefreshToken { uuid: Uuid::new() })
            .map_err(|_| err!(Status::InternalServerError, "error generating refresh_id"))?
            .inner;

        existing_user_token.inner.refresh_id = Some(new_refresh_token.uuid);

        let updated_user_token = UserTokenController
            .update(
                &existing_user_token.inner,
                Box::new(user_tokens::id.eq(existing_user_token.id)),
            )
            .map_err(|_| err!(Status::InternalServerError, "error updating user token"))?;

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &JWT::new(), b"secret")
            .map_err(|_| err!(Status::InternalServerError, "error creating jsonwebtoken"))?;

        let access_token = AccessTokenController
            .create(&AccessToken {
                jwt,
                expires_in: 3600,
                user_id: updated_user_token.id,
            })
            .map_err(|_| err!(Status::InternalServerError, "error creating access token"))?;

        Ok(json!({
            "token_type": "Bearer",
            "access_token": &access_token.inner.jwt,
            "expires_in": &access_token.inner.expires_in,
            "refresh_token": &updated_user_token.inner.refresh_id.as_ref().unwrap(),
        }))
    }
}
