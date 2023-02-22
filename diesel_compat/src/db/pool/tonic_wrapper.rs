use super::pool::PoolConn;
use lazy_static::lazy_static;

use super::pool::TerpDbPool;

pub struct Pooler {
    pub raw_conn: TerpDbPool,
}

impl Pooler {
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

pub type TonicPoolInner = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
