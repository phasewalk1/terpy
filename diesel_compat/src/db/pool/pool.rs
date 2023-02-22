use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type TerpDbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PoolConn<T> = r2d2::PooledConnection<T>;
