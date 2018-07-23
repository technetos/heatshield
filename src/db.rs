use diesel::{pg::PgConnection, Connection};
use std::{env, error::Error};

pub fn establish_connection() -> PgConnection {
    let env_var = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&env_var[..]).unwrap()
}
