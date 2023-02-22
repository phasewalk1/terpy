use prostgen::user::user_service_server::UserServiceServer;
use tonic::transport::Server;
use user::prelude::*;

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
