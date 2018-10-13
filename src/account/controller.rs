use crate::{
    account::model::{Account, AccountWithId},
    schema::accounts::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};
use postgres_resource::{self, controller::*};
use rocket_contrib::{Json, Value};

resource_controller!(Account);
