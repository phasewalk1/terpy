#[macro_use]
extern crate diesel;
pub mod auto;
pub use auto::test_results as test_results_t;
pub use auto::users as user_t;
pub mod db;
pub mod models;