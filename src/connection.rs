/*extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub type Conn = SqliteConnection;

pub fn get_database_url_dot_env() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish() -> SqliteConnection {
    let database_url = get_database_url_dot_env();
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
pub fn establish_with_url(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
pub fn r_establish() -> Result<SqliteConnection, diesel::ConnectionError> {
    let database_url = get_database_url_dot_env();
    let conn = SqliteConnection::establish(&database_url)?;
    Ok(conn)
}
*/

use diesel::sqlite::SqliteConnection;
use r2d2;
use diesel::r2d2::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use dotenv::dotenv;
use std::env;
use std::ops::Deref;

lazy_static! {
    pub static ref DB_POOL: Pool = init_pool();
}

pub type Conn = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Conn>>;

pub fn get_database_url_dot_env() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<Conn>::new(get_database_url_dot_env());
    Pool::new(manager).expect("db pool")
}



pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<Conn>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = Conn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
