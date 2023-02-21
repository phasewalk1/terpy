#[macro_use]
extern crate diesel;
pub mod models;

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
