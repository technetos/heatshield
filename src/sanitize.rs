use rocket_contrib::Value;
use postgres_resource::{self, controller::*};

pub trait Sanitizer
where
    Self: ResourceWithId + Resource,
{
    fn sanitize(&self, model: &Self::Model) -> Result<(), Value>;
}
