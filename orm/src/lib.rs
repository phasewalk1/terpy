#[macro_use]
extern crate diesel;

/// Diesel ORM auto-generated code
pub mod auto;
/// Database operations and pooling
pub mod db;
/// Diesel ORM models
pub mod models;

/// Diesel ORM tables
pub mod tables {
    pub use super::auto::cannibanoid_screen as cannibanoid_screen_t;
    pub use super::auto::terpenoid_screen as terpenoid_screen_t;
    pub use super::auto::test_results_t;
    pub use super::auto::users as user_t;
}

/// Re-exports for the grower module
pub mod prelude_grower {
    pub use super::models::grower::{
        InsertableCannibanoidScreen, InsertableTerpenoidScreen, InsertableTestResults,
        SearchableCannibanoidScreen, SearchableTerpenoidScreen, SearchableTestResults,
    };
}

/// Re-exports for the user module
pub mod prelude_user {
    pub use super::models::user::{InsertableUser, SearchableUser};
}

/// Database pool re-exports
pub mod prelude_pool {
    pub use super::db::pooling::rocket_wrapper::RocketPoolGuard as GUARD_POOL_R;
    pub use super::db::pooling::rocket_wrapper::ROCKET_POOL as POOL_R;
    pub use super::db::pooling::tonic_wrapper::TONIC_POOL as POOL_T;
}

/// Global re-exports
pub mod prelude {
    pub use super::prelude_grower::*;
    pub use super::prelude_pool::*;
    pub use super::prelude_user::*;
    pub use super::tables::*;
}
