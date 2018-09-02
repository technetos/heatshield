use client_token::{
    controller::ClientTokenController,
    model::{ClientToken, ClientTokenWithId},
};
use controller::ResourceController;
use diesel::ExpressionMethods;
use jsonwebtoken;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::{Json, Value};
use schema;
use user_token::{
    controller::UserTokenController,
    model::{UserToken, UserTokenWithId},
};
use uuid::Uuid;

pub struct Bearer {
    pub auth: String,
}

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

        println!("{}", token);

        let decoded = jsonwebtoken::decode::<UserToken>(
            token,
            b"secret",
            &jsonwebtoken::Validation::default(),
        ).unwrap();

        let mut _token = UserTokenController.get_one(Box::new(
            schema::user_tokens::refresh_id.eq(decoded.claims.refresh_id),
        ));

        if let Ok(ref mut dirty_token) = _token {
            dirty_token.user_token.refresh_id = Uuid::new_v4();
            let new_token = UserTokenController.update(
                &dirty_token.user_token,
                Box::new(schema::user_tokens::id.eq(dirty_token.id)),
            );

            let _jwt = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &new_token.unwrap().user_token,
                b"secret",
            );

            if let Ok(jwt) = _jwt {
                return Outcome::Success(Bearer { auth: jwt });
            } else {
                return Outcome::Failure((
                    Status::BadRequest,
                    Json(json!("error creating jsonwebtoken")),
                ));
            }
        } else {
            return Outcome::Failure((Status::BadRequest, Json(json!("Invalid token"))));
        }
    }
}
