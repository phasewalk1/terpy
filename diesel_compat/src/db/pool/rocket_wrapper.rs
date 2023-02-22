use super::pool::TerpDbPool;
use diesel::r2d2::{self, ConnectionManager};
use lazy_static::lazy_static;
use std::env::var as env;

lazy_static! {
    pub static ref DATABASE_URL: String = env("DATABASE_URL").expect("DATABASE_URL must be set");
}

pub fn init_pool() -> TerpDbPool {
    let mgr = diesel::r2d2::ConnectionManager::<diesel::pg::PgConnection>::new(&*DATABASE_URL);
    return TerpDbPool::builder()
        .max_size(10)
        .build(mgr)
        .expect("Failed to create pool.");
}

pub struct RocketPoolGuard(pub r2d2::PooledConnection<ConnectionManager<diesel::pg::PgConnection>>);

impl From<r2d2::PooledConnection<ConnectionManager<diesel::pg::PgConnection>>> for RocketPoolGuard {
    fn from(conn: r2d2::PooledConnection<ConnectionManager<diesel::pg::PgConnection>>) -> Self {
        Self(conn)
    }
}

use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::State;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RocketPoolGuard {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let outcome = request
            .guard::<&State<TerpDbPool>>()
            .await
            .map(|pool| pool.inner().clone());
        match outcome {
            Outcome::Success(pool) => match pool.get() {
                Ok(conn) => Outcome::Success(RocketPoolGuard(conn)),
                Err(_) => Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
            },
            Outcome::Forward(_) => Outcome::Forward(()),
            Outcome::Failure((status, _)) => Outcome::Failure((status, ())),
        }
    }
}

impl std::ops::Deref for RocketPoolGuard {
    type Target = diesel::pg::PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
