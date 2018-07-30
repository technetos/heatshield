use controller::ResourceControllerLifecycle;
use rocket::http::Method::*;
use rocket::Route;
use rocket_contrib::Json;
use serde::{Deserialize, Serialize};

pub trait Action<'a, Model, ModelWithId>
where
    Model: Deserialize<'a>,
    ModelWithId: Serialize,
    Self: ResourceControllerLifecycle,
{
    fn create(&mut self, model: Json<Model>) -> Json<ModelWithId>;
    //  fn get_all(&mut self) -> Json<Vec<ModelWithId>>;
    fn get_one(&mut self, id: i32) -> Json<ModelWithId>;
    //  fn update(&mut self, model: Json<ModelWithId>) -> Json<ModelWithId>;
    //  fn delete(&mut self, model: Json<ModelWithId>) -> bool;
}
