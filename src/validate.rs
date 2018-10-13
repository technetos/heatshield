use crate::controller::Resource;
use rocket_contrib::Json;

pub trait Validator {
    fn validate(&self) -> Result<(), Json>;
}
