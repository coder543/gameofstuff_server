
use r2d2;
use postgres::Connection;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// An alias to the type for a pool of postgresql connections.
type Pool = r2d2::Pool<PostgresConnectionManager>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<PostgresConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using &DbConn as &Connection.
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager =
        PostgresConnectionManager::new("postgres://coder:coder@localhost/coder", TlsMode::None)
            .unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    pool
}