use db::establish_connection as connection;
use diesel::{
    self, expression::BoxableExpression, insert_into, pg::Pg, prelude::*, result::Error,
    sql_types::Bool,
};

pub enum ControllerKind {
    Simple(Box<ControllerLifecycle>),
    Resource(Box<ResourceControllerLifecycle>),
}

pub trait ControllerLifecycle {
    fn before(&self) {}
    fn execute(&self) {}
    fn after(&self) {}
}

pub trait ResourceControllerLifecycle {
    fn _create(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn get_all(&mut self) {}
    fn _get_one(&mut self) {}
}

pub trait ResourceController<Model, ModelWithId, DBTable, SQLType>
where
    Model: Insertable<DBTable>,
    ModelWithId: Queryable<SQLType, Pg>,
    DBTable: diesel::Table,
{
    fn __create(&self, model: &Model) -> Result<ModelWithId, Error>;

    fn __get_one(
        &self,
        by: &Fn(&ModelWithId) -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<ModelWithId, Error>;

    fn __get_all(
        &self,
        by: &Fn(&ModelWithId) -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<Vec<ModelWithId>, Error>;
}
