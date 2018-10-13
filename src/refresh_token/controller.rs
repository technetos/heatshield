use crate::{
    refresh_token::model::{RefreshToken, RefreshTokenWithId},
    schema::refresh_tokens::{table, SqlType},
};

use postgres_resource::{self, controller::*};
use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(RefreshToken);
