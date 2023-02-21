use prostgen::prostgen::*;
use tonic::{Request, Response, Status};

pub mod prelude {
    use super::*;
    #[derive(Debug, Default)]
    pub struct UserService {}

    #[tonic::async_trait]
    impl UserServiceTrait for UserService {
        async fn user_by_name(
            &self,
            request: Request<UserByNameRequest>,
        ) -> Result<Response<User>, Status> {
            todo!()
        }
        async fn user_by_email(
            &self,
            request: Request<UserByEmailRequest>,
        ) -> Result<Response<User>, Status> {
            todo!()
        }
        async fn create_user(&self, request: Request<NewUser>) -> Result<Response<User>, Status> {
            todo!()
        }
        async fn update_user(
            &self,
            request: Request<UpdateUserRequest>,
        ) -> Result<Response<User>, Status> {
            todo!()
        }
        async fn delete_user(
            &self,
            request: Request<DeleteUserRequest>,
        ) -> Result<Response<()>, Status> {
            todo!()
        }
    }
}
