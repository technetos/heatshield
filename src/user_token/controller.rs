use crate::{
    schema::user_tokens::{table, SqlType},
    user_token::model::{UserToken, UserTokenWithId},
};

use diesel::{insert_into, prelude::*, result::Error, update};
use postgres_resource::{self, controller::*};

resource_controller!(UserToken);
