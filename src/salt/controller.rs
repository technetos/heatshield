use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use salt::model::{Salt, SaltWithId};
use schema::salts::{table, SqlType};

resource_controller!(Salt);
