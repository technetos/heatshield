use postgres_resource::{self, controller::*};
use rocket_contrib::Value;

pub trait Sanitizer
where
    Self: ResourceWithId + Resource,
{
    fn sanitize(&self, model: &Self::Model) -> Result<(), Value>;
}
