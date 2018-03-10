extern crate r2d2;

use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// An alias to the type for a pool of Diesel SQLite connections.
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

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
  let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
  r2d2::Pool::new(manager).expect("db pool")
}