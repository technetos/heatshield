use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use schema::user_tokens::{table, SqlType};
use user_token::model::{UserToken, UserTokenWithId};

resource_controller!(UserToken);
