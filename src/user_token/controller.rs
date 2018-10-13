use crate::{
    schema::user_tokens::{table, SqlType},
    user_token::model::{UserToken, UserTokenWithId},
};

use postgres_resource::{self, controller::*};
use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(UserToken);
