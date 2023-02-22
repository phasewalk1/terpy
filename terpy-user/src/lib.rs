use diesel_compat::models::user::{InsertableUser, SearchableUser};
use prostgen::user::user_service_server::UserService as UserServiceTrait;
use prostgen::user::*;
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
        async fn create_user(&self, request: Request<NewUser>) -> Result<Response<User>, Status> {
            let req = decompose_request(request);
            let new_user: InsertableUser<'_> = req.into();
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
        async fn user_by_name(
            &self,
            request: Request<UserByNameRequest>,
        ) -> Result<Response<User>, Status> {
            let req = decompose_request(request);
            let maybe_conn = diesel_compat::db::pool::tonic_wrapper::TONIC_POOL.try_connect();
            match maybe_conn {
                Ok(conn) => {
                    if let Ok(u) = SearchableUser::by_name(req.name, conn) {
                        if let Some(user) = u {
                            return Ok(tonic::Response::new(user.into()));
                        } else {
                            return Err(tonic::Status::not_found("User not found"));
                        }
                    } else {
                        return Err(tonic::Status::internal("Error finding user"));
                    }
                },
                Err(e) => return Err(e),
            }
        }
        async fn user_by_id(
            &self,
            request: Request<UserByIdRequest>,
        ) -> Result<Response<User>, Status> {
            let req = decompose_request(request);
            let maybe_conn = diesel_compat::db::pool::tonic_wrapper::TONIC_POOL.try_connect();
            match maybe_conn {
                Ok(conn) => {
                    let id = req.id.parse::<i32>().unwrap_or(-1);
                    if id == -1 {
                        return Err(tonic::Status::invalid_argument("Invalid id"));
                    }                    
                    if let Ok(u) = SearchableUser::by_id(id, conn) {
                        if let Some(user) = u {
                            return Ok(tonic::Response::new(user.into()));
                        } else {
                            return Err(tonic::Status::not_found("User not found"));
                        }
                    } else {
                        return Err(tonic::Status::internal("Error finding user"));
                    }
                },
                Err(e) => return Err(e),
            }
        }
        async fn user_by_email(
            &self,
            request: Request<UserByEmailRequest>,
        ) -> Result<Response<User>, Status> {
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
