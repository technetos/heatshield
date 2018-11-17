use postgres_resource::*;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::JsonValue;

pub trait Validator {
    fn validate(&self) -> Result<(), Custom<JsonValue>>;
}
