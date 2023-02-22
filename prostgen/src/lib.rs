pub mod pb;
pub use pb::grower;
pub use pb::user;

pub mod clients {
    pub use super::grower::grower_client::GrowerClient;
    pub use super::user::user_service_client::UserServiceClient;
}

pub mod services {
    pub use super::grower::grower_server::Grower;
    pub use super::user::user_service_server::UserService;
}

pub mod prelude {
    pub use super::clients::*;
    pub use super::prelude_grow::*;
    pub use super::prelude_usr::*;
    pub use super::services::*;
}

pub mod prelude_usr {
    pub use super::user::{
        DeleteUserRequest, NewUser, UpdateUserRequest, User, UserByEmailRequest, UserByNameRequest,
    };
}
pub mod prelude_grow {
    pub use super::grower::{
        CannibanoidScreen, NewTestResults, TerpenoidScreen, TestResultById, TestResults,
    };
}
