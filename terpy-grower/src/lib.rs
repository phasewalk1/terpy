use prostgen::services::Grower;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GrowerService {}

#[tonic::async_trait]
impl Grower for GrowerService {
    async fn create_test_results(
        &self,
        request: Request<prostgen::grower::NewTestResults>,
    ) -> Result<Response<prostgen::grower::TestResults>, Status> {
        todo!()
    }

    async fn get_test_results(
        &self,
        request: Request<prostgen::grower::TestResultById>,
    ) -> Result<Response<prostgen::grower::TestResults>, Status> {
        todo!()
    }
}
