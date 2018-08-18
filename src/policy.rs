use controller::ResourceController;
use diesel::ExpressionMethods;
use jsonwebtoken;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::{Json, Value};
use schema;
use token::{controller::AccessTokenController, model::AccessToken};

pub struct Bearer;

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

        let decoded = jsonwebtoken::decode::<AccessToken>(
            token,
            b"00000000-0000-0000-0000-000000000000",
            &jsonwebtoken::Validation::default(),
        );

        Outcome::Success(Bearer)
    }
}
