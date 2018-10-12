use controller::{Expr, Resource, ResourceController, ResourceSql, ResourceTable, ResourceWithId};
use diesel::{insert_into, prelude::*, result::Error, update};
use refresh_token::model::{RefreshToken, RefreshTokenWithId};
use schema::refresh_tokens::{table, SqlType};

resource_controller!(RefreshToken);
