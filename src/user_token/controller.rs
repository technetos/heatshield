use crate::{
    controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId},
    schema::user_tokens::{table, SqlType},
    user_token::model::{UserToken, UserTokenWithId},
};

use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(UserToken);
