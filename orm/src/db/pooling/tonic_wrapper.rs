use super::pool::PoolConn;
use lazy_static::lazy_static;

use super::pool::TerpDbPool;

/// Pool builder
pub struct Pooler {
    pub raw_conn: TerpDbPool,
}

impl Pooler {
    /// Try to get a connection from the pool.
    pub fn try_connect(&self) -> Result<PoolConn<TonicPoolInner>, tonic::Status> {
        let maybe_conn = self.raw_conn.get();
        match maybe_conn {
            Ok(conn) => return Ok(conn),
            Err(_) => {
                return Err(tonic::Status::internal(format!(
                    "Error getting connection from Db pool"
                )))
            }
        }
    }
}

lazy_static! {
    /// A static reference to the connection pool.
    pub static ref TONIC_POOL: Pooler = {
        Pooler {
            raw_conn: TerpDbPool::builder()
                .max_size(10)
                .build(diesel::r2d2::ConnectionManager::new(
                    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                ))
                .expect("Failed to create TONIC_POOL"),
        }
    };
}

/// A raw connection
pub type TonicPoolInner = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
