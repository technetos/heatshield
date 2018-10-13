use crate::{
    client::model::{Client, ClientWithId},
    controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId},
    schema::clients::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(Client);
