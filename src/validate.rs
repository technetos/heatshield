use postgres_resource::*;
use rocket_contrib::Json;
use rocket::{response::status::Custom, http::Status};

pub trait Validator {
    fn validate(&self) -> Result<(), Custom<Json>>;
}
