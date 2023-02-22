use prostgen::services::Grower;
use terpy_orm::{prelude_grower::*, prelude_pool::POOL_T};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GrowerService {}

#[tonic::async_trait]
impl Grower for GrowerService {
    /// (RPC) Create a new cannibanoid screen
    async fn create_cannibanoid_screen(
        &self,
        request: Request<prostgen::grower::NewCannibanoidScreen>,
    ) -> Result<Response<prostgen::grower::CannibanoidScreen>, Status> {
        let screen: InsertableCannibanoidScreen = request.into_inner().into();
        let maybe_conn = POOL_T.try_connect();
        match maybe_conn {
            Ok(conn) => {
                if let Ok(res) = screen.insert(conn) {
                    return Ok(tonic::Response::new(res.into()));
                } else {
                    return Err(tonic::Status::internal("Error inserting screen into db"));
                }
            }
            Err(e) => return Err(e),
        }
    }

    /// (RPC) Create a new terpenoid screen
    async fn create_terpenoid_screen(
        &self,
        request: Request<prostgen::grower::NewTerpenoidScreen>,
    ) -> Result<Response<prostgen::grower::TerpenoidScreen>, Status> {
        let screen: InsertableTerpenoidScreen = request.into_inner().into();
        todo!()
    }

    /// (RPC) Get a cannibanoid screen by id
    async fn get_cannibanoid_screen(
        &self,
        request: Request<prostgen::grower::CannibanoidScreenById>,
    ) -> Result<Response<prostgen::grower::CannibanoidScreen>, Status> {
        todo!()
    }

    /// (RPC) Get a terpenoid screen by id
    async fn get_terpenoid_screen(
        &self,
        request: Request<prostgen::grower::TerpenoidScreenById>,
    ) -> Result<Response<prostgen::grower::TerpenoidScreen>, Status> {
        todo!()
    }

    /// (RPC) Create a new test result
    async fn create_test_results(
        &self,
        request: Request<prostgen::grower::NewTestResults>,
    ) -> Result<Response<prostgen::grower::TestResults>, Status> {
        todo!()
    }

    /// (RPC) Get a test result by id
    async fn get_test_results(
        &self,
        request: Request<prostgen::grower::TestResultById>,
    ) -> Result<Response<prostgen::grower::TestResults>, Status> {
        todo!()
    }

    /// (RPC) Assign a test result to a screen
    async fn assign_test_results(
        &self,
        request: Request<prostgen::grower::AssignTestResultsRequest>,
    ) -> Result<Response<prostgen::grower::TestResults>, Status> {
        todo!()
    }
}
