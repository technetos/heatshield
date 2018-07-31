use db::establish_connection as connection;
use diesel::{
    self, expression::BoxableExpression, pg::Pg, prelude::*, result::Error, sql_types::Bool,
};

pub trait ResourceController<Model, ModelWithId, DBTable, SQLType>
where
    Model: Insertable<DBTable>,
    ModelWithId: Queryable<SQLType, Pg>,
    DBTable: diesel::Table,
{
    fn _create(&self, model: &Model) -> Result<ModelWithId, Error>;

    fn _get_one(
        &self,
        by: &Fn() -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<ModelWithId, Error>;

    fn _get_all(
        &self,
        by: &Fn() -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<Vec<ModelWithId>, Error>;

    fn _update(
        &self,
        model: &Model,
        by: &Fn() -> Box<BoxableExpression<DBTable, Pg, SqlType = Bool>>,
    ) -> Result<ModelWithId, Error>;
}
