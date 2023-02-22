use grower::*;
use prostgen::grower::grower_server::GrowerServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "50051".to_string())
        .parse()
        .unwrap_or_else(|_| 50051);
    let addr = format!("[::1]:{}", port).parse().unwrap();
    let grower_service = GrowerService::default();
    pretty_env_logger::try_init().ok();
    log::info!("Starting server on {}", addr);

    Server::builder()
        .add_service(GrowerServer::new(grower_service))
        .serve(addr)
        .await
        .unwrap();
}
