use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

/// A type alias for a connection pool.
pub type TerpDbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
/// A generic connection pool.
pub type PoolConn<T> = r2d2::PooledConnection<T>;
