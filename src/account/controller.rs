use account::model::{Account, AccountWithId};
use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use rocket_contrib::{Json, Value};
use schema::accounts::{table, SqlType};
use verification::model::Verification;

resource_controller!(Account);
