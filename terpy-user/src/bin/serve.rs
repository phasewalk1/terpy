use prostgen::user::user_service_server::UserServiceServer;
use tonic::transport::Server;
use user::prelude::*;

#[tokio::main]
async fn main() {
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "50051".to_string())
        .parse()
        .unwrap_or_else(|_| 50051);
    let addr = format!("[::1]:{}", port).parse().unwrap();
    let user_service = UserService::default();
    pretty_env_logger::try_init().ok();
    log::info!("Starting server on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await
        .unwrap();
}
