use crate::{
    account::model::{Account, AccountWithId},
    controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId},
    schema::accounts::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};
use rocket_contrib::{Json, Value};

resource_controller!(Account);
