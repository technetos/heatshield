use crate::{
    access_token::{AccessToken, AccessTokenController, AccessTokenWithId},
    jwt::JWT,
    refresh_token::{RefreshToken, RefreshTokenController, RefreshTokenWithId},
    schema,
    user_token::{UserToken, UserTokenController, UserTokenWithId},
};

use compat_uuid::Uuid;
use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{
    fairing,
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use rocket_contrib::json::JsonValue;

pub struct Bearer(pub UserTokenWithId);

fn is_valid(parts: &Vec<&str>) -> bool {
    if let Some(part) = parts.first() {
        return parts.len() == 2 && part == &"Bearer";
    }
    false
}

impl<'a, 'r> FromRequest<'a, 'r> for Bearer {
    type Error = JsonValue;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Bearer, JsonValue> {
        let keys: Vec<_> = request.headers().get("authorization").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, json!({})));
        }

        let parts: Vec<&str> = keys[0].split(" ").collect();

        if !is_valid(&parts) {
            return Outcome::Failure((Status::BadRequest, json!("Invalid authorization scheme")));
        }

        let token = String::from(parts[1]);

        let jwt = jsonwebtoken::decode::<JWT>(
            &token,
            "secret".as_ref(),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| match e {
            _ => Err((Status::Unauthorized, json!("Invalid access token"))),
        })?;

        let access_token = AccessTokenController
            .get_one(Box::new(schema::access_tokens::jwt.eq(token)))
            .map_err(|_| Err((Status::Unauthorized, json!("Invalid access token"))))?;

        let user_token = UserTokenController
            .get_one(Box::new(
                schema::user_tokens::id.eq(access_token.access_token.user_id),
            ))
            .map_err(|_| Err((Status::Unauthorized, json!("Invalid access token"))))?;

        Outcome::Success(Bearer(user_token))
    }
}
