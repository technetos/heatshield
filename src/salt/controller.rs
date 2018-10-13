use crate::{
    salt::model::{Salt, SaltWithId},
    schema::salts::{table, SqlType},
};

use postgres_resource::{self, controller::*};
use diesel::{insert_into, prelude::*, result::Error, update};

resource_controller!(Salt);
