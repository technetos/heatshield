use postgres_resource::*;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::Json;

pub trait Validator {
    fn validate(&self) -> Result<(), Custom<Json>>;
}
