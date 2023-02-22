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

pub mod extension_user_by_id {
    #[rocket::async_trait]
    impl<'r> rocket::request::FromRequest<'r> for crate::user::UserByIdRequest {
        type Error = ();
        async fn from_request(
            request: &'r rocket::request::Request<'_>,
        ) -> rocket::request::Outcome<Self, Self::Error> {
            // get the user id from the request
            let id = request
                .headers()
                .get_one("id")
                .unwrap_or("-1")
                .parse::<i32>()
                .unwrap_or(-1);
            match id {
                -1 => rocket::request::Outcome::Failure((rocket::http::Status::BadRequest, ())),
                _ => rocket::request::Outcome::Success(crate::user::UserByIdRequest {
                    id: id.to_string(),
                }),
            }
        }
    }
}
