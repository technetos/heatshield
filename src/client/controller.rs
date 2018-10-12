use client::model::{Client, ClientWithId};
use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use schema::clients::{table, SqlType};

resource_controller!(Client);
