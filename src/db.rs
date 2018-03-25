extern crate r2d2;
extern crate dotenv;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;
use std::env;

// An alias to the type for a pool of Diesel SQLite connections.
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DB(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection >>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DB {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
    let pool = request.guard::<State<Pool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(DB(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
    }
  }
}

// For the convenience of using an &DB as an &MysqlConnection.
impl Deref for DB {
  type Target = MysqlConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// Initializes a database pool.
pub fn init_pool() -> Pool {
  dotenv::dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<MysqlConnection>::new(database_url);
  r2d2::Pool::new(manager).expect("db pool")
}