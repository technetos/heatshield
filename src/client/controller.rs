use crate::{
    client::model::{Client, ClientWithId},
    schema::clients::{table, SqlType},
};

use postgres_resource::{self, controller::*};
use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(Client);
