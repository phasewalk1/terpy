pub mod pb;
pub mod prostgen {
    pub use super::pb::user::{
        user_service_client::UserServiceClient,
        user_service_server::{UserService as UserServiceTrait, UserServiceServer},
    };
    pub use super::pb::user::{
        DeleteUserRequest, NewUser, UpdateUserRequest, User, UserByEmailRequest, UserByNameRequest,
    };
}
