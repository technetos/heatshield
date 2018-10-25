use crate::{
    salt::model::{Salt, SaltWithId},
    schema::salts::{table, SqlType},
};

use diesel::{insert_into, prelude::*, result::Error, update};
use postgres_resource::{self, controller::*};

resource_controller!(Salt);
