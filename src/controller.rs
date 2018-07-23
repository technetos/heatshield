use db::establish_connection as connection;
use diesel::{self, insert_into, prelude::*};

pub trait ControllerLifecycle {
  fn before_create(&mut self);
  fn create(&mut self);
  fn after_create(&mut self);
}

pub trait ResourceController<Model, ModelWithId, DBTable, SQLType>
where
    Model: Insertable<DBTable>,
    ModelWithId: Queryable<SQLType, diesel::pg::Pg>,
    DBTable: diesel::Table,
{
    fn create_resource(&self, model: &Model) -> Result<ModelWithId, diesel::result::Error>;
    fn select_resource(&self, model: &ModelWithId) -> Result<(), diesel::result::Error>;
}
