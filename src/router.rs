use controller::ResourceController;
use failure::Error;
use rocket_contrib::Json;

pub const BASEPATH: &'static str = "/heatsheild/v1";

pub trait Action
where
    Self: ResourceController,
{
    fn create(&self, model: Json<Self::Model>) -> Result<Json<Self::ModelWithId>, Error>;

    fn get_one(&self, id: i32) -> Result<Json<Self::ModelWithId>, Error>;

    fn update(&self, model: Json<Self::ModelWithId>) -> Result<Json<Self::ModelWithId>, Error>;
    //  fn delete(&mut self, model: Json<ModelWithId>) -> bool;
}

pub trait GetAllActionExt: Action {
    fn get_all(&self) -> Json<Vec<Self::ModelWithId>>;
}
