use controller::ResourceController;
use diesel::{self, pg::Pg, prelude::*};
use failure;
use rocket_contrib::Json;
use serde::{Deserialize, Serialize};

pub trait Action<'a, Model, ModelWithId, DBTable, SQLType>
where
    DBTable: diesel::Table,
    Model: Deserialize<'a> + Insertable<DBTable>,
    ModelWithId: Serialize + Queryable<SQLType, Pg>,
    Self: ResourceController<Model, ModelWithId, DBTable, SQLType>,
{
    fn create(&mut self, model: Json<Model>) -> Result<Json<ModelWithId>, failure::Error>;
    fn get_one(&mut self, id: i32) -> Result<Json<ModelWithId>, failure::Error>;
    fn update(&mut self, model: Json<ModelWithId>) -> Result<Json<ModelWithId>, failure::Error>;
    //  fn delete(&mut self, model: Json<ModelWithId>) -> bool;
}

pub trait GetAllActionExt<'a, Model, ModelWithId, DBTable, SQLType>:
    Action<'a, Model, ModelWithId, DBTable, SQLType>
where
    DBTable: diesel::Table,
    Model: Deserialize<'a> + Insertable<DBTable>,
    ModelWithId: Serialize + Queryable<SQLType, Pg>,
{
    fn get_all(&mut self) -> Json<Vec<ModelWithId>>;
}
