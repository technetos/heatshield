use crate::{
    client::model::{Client, ClientWithId},
    schema::clients::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};
use postgres_resource::{self, controller::*};

resource_controller!(Client);
