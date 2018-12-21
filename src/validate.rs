use rocket::response::status::Custom;
use rocket_contrib::json::JsonValue;

pub trait Validator {
    fn validate(&self) -> Result<(), Custom<JsonValue>>;
}
