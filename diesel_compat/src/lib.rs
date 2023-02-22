#[macro_use]
extern crate diesel;
pub mod db;
pub mod models;

pub use models::user::{NewUser as NewUserCompat, User as UserCompat};

pub use self::diesel_schema::users as user_t;
pub mod diesel_schema {
    diesel::table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
            email -> Varchar,
            password_hash -> Varchar,
        }
    }
}
