use db::establish_connection as connection;
use diesel::{
    self, expression::BoxableExpression, insert_into, pg::Pg, prelude::*, result::Error,
    sql_types::Bool,
};

pub trait ControllerLifecycle {
    fn before_create(&mut self) {}
    fn create(&mut self) {}
    fn after_create(&mut self) {}

    fn before_get_all(&mut self) {}
    fn get_all(&mut self) {}
    fn after_get_all(&mut self) {}

    fn before_get_one(&mut self) {}
    fn get_one(&mut self) {}
    fn after_get_one(&mut self) {}
}

pub trait ResourceController<Model, ModelWithId, DBTable, SQLType>
where
    Model: Insertable<DBTable>,
    ModelWithId: Queryable<SQLType, Pg>,
    DBTable: diesel::Table,
{
    fn _create(&self, model: &Model) -> Result<ModelWithId, Error>;

    fn _get_one(
        &self,
        by: &Fn(&ModelWithId) -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<ModelWithId, Error>;

    fn _get_all(
        &self,
        by: &Fn(&ModelWithId) -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<Vec<ModelWithId>, Error>;
}
