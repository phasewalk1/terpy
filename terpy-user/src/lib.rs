use diesel_compat::{NewUserCompat, UserCompat};
use prostgen::prostgen::*;
use tonic::{Request, Response, Status};

pub mod prelude {
    use super::*;
    #[derive(Debug, Default)]
    pub struct UserService {}

    fn decompose_request<T>(request: tonic::Request<T>) -> T
    where
        T: prost::Message + Clone,
    {
        let req = request.into_inner();
        return req;
    }

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
            let req = decompose_request(request);
            let new_user: NewUserCompat<'_> = req.into();
            let maybe_conn = diesel_compat::db::pool::tonic_wrapper::TONIC_POOL.try_connect();
            match maybe_conn {
                Ok(conn) => {
                    if let Ok(res) = new_user.insert(conn) {
                        return Ok(tonic::Response::new(res.into()));
                    } else {
                        return Err(tonic::Status::internal("Error inserting user into db"));
                    }
                }
                Err(e) => return Err(e),
            }
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
