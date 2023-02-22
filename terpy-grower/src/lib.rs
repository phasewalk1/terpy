use diesel_compat::models::grower::{
    NewTestResults as NewTestResultsCompat, TestResults as TestResultsCompat,
};
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
        let new_test_results = NewTestResultsCompat::from(request.into_inner());
        todo!()
    }

    async fn get_test_results(
        &self,
        request: Request<prostgen::grower::TestResultById>,
    ) -> Result<Response<prostgen::grower::TestResults>, Status> {
        todo!()
    }
}
