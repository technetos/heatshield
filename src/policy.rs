use crate::{
    refresh_token::{RefreshToken, RefreshTokenController, RefreshTokenWithId},
    schema,
    user_token::{UserToken, UserTokenController, UserTokenWithId},
};

use diesel::ExpressionMethods;
use jsonwebtoken;
use postgres_resource::ResourceController;
use rocket::{
    fairing,
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use rocket_contrib::{Json, Value};
use uuid::Uuid;

pub struct Bearer(UserToken);

fn is_valid(parts: &Vec<&str>) -> bool {
    if let Some(part) = parts.first() {
        return parts.len() == 2 && part == &"Bearer";
    }
    false
}

impl<'a, 'r> FromRequest<'a, 'r> for Bearer {
    type Error = Json;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Bearer, Json> {
        let keys: Vec<_> = request.headers().get("authorization").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, Json(json!({}))));
        }

        let parts: Vec<&str> = keys[0].split(" ").collect();

        if !is_valid(&parts) {
            return Outcome::Failure((
                Status::BadRequest,
                Json(json!("Invalid authorization scheme")),
            ));
        }

        let token = parts[1];

        let jwt = jsonwebtoken::decode::<UserToken>(
            token,
            b"secret",
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| match e {
            _ => Err((Status::Unauthorized, Json(json!("Invalid token")))),
        })?;

        let refresh_token = RefreshTokenController
            .get_one(Box::new(schema::refresh_tokens::uuid.eq(jwt.claims.refresh_id.unwrap())))
            .map_err(|e| match e {
                _ => Err((Status::Unauthorized, Json(json!("Invalid token")))),
            })?
            .refresh_token;

        let user_token = UserTokenController
            .get_one(Box::new(schema::user_tokens::refresh_id.eq(refresh_token.uuid)))
            .map_err(|e| match e {
                _ => Err((Status::Unauthorized, Json(json!("Invalid token")))),
            })?
            .user_token;

        Outcome::Success(Bearer(user_token))
    }
}
