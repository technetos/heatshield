use crate::{
    refresh_token::model::{RefreshToken, RefreshTokenWithId},
    schema::refresh_tokens::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};
use postgres_resource::{self, controller::*};

resource_controller!(RefreshToken);
