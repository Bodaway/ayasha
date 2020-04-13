extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn get_database_url_dot_env() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set")
}

pub fn establish() -> SqliteConnection {
    let database_url = get_database_url_dot_env();
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
pub fn establish_with_url(database_url : &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
