use tonic::{service::Interceptor, Request, Status};

pub struct UserAuthInterceptor {
    api_key: String,
}

impl UserAuthInterceptor {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[tonic::async_trait]
impl Interceptor for UserAuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();
        let api_key = metadata.get("api-key").unwrap().to_str().unwrap();
        if api_key != self.api_key {
            return Err(Status::unauthenticated("Invalid API key"));
        }
        Ok(request)
    }
}
