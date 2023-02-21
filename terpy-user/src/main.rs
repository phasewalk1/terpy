use terpy_core::prostgen::*;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct UserService {}

#[tonic::async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(&self, request: Request<UserRequest>) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn delete_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn get_user_by_email(
        &self,
        request: Request<UserByEmailRequest>,
    ) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn list_users(&self, request: Request<User>) -> Result<Response<UserList>, Status> {
        todo!()
    }

    async fn update_user(&self, request: Request<UserRequest>) -> Result<Response<User>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let user_service = UserService::default();
    pretty_env_logger::try_init().ok();
    log::info!("Starting server on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await
        .unwrap();
}
