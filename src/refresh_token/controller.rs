use crate::{
    controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId},
    refresh_token::model::{RefreshToken, RefreshTokenWithId},
    schema::refresh_tokens::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(RefreshToken);
