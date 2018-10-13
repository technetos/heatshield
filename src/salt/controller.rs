use crate::{
    controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId},
    salt::model::{Salt, SaltWithId},
    schema::salts::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(Salt);
