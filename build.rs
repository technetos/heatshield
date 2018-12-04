use std::{env, io};
use diesel::*;
use dotenv::dotenv;

fn main() {
    let env_var = std::env::var("DATABASE_URL").expect(&format!("DATABASE_URL not set"));
    let connection = PgConnection::establish(&env_var[..]).expect("Unable to establish connection to database");

    let migrations_dir = diesel_migrations::find_migrations_directory().unwrap();
    println!("cargo:rerun-if-changed={}", migrations_dir.display());
    diesel_migrations::run_pending_migrations_in_directory(&connection, &migrations_dir, &mut io::sink()).unwrap();
}
