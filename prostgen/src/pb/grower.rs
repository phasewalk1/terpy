#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CannibanoidScreen {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub cbc: f32,
    #[prost(float, tag = "5")]
    pub cbd: f32,
    #[prost(float, tag = "6")]
    pub cbda: f32,
    #[prost(float, tag = "7")]
    pub cbdv: f32,
    #[prost(float, tag = "8")]
    pub cbg: f32,
    #[prost(float, tag = "9")]
    pub cbga: f32,
    #[prost(float, tag = "10")]
    pub cbn: f32,
    #[prost(float, tag = "11")]
    pub d9thc: f32,
    #[prost(float, tag = "12")]
    pub d8thc: f32,
    #[prost(float, tag = "13")]
    pub thcv: f32,
    #[prost(float, tag = "14")]
    pub thca: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCannibanoidScreen {
    #[prost(string, tag = "1")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(float, tag = "3")]
    pub cbc: f32,
    #[prost(float, tag = "4")]
    pub cbd: f32,
    #[prost(float, tag = "5")]
    pub cbda: f32,
    #[prost(float, tag = "6")]
    pub cbdv: f32,
    #[prost(float, tag = "7")]
    pub cbg: f32,
    #[prost(float, tag = "8")]
    pub cbga: f32,
    #[prost(float, tag = "9")]
    pub cbn: f32,
    #[prost(float, tag = "10")]
    pub d9thc: f32,
    #[prost(float, tag = "11")]
    pub d8thc: f32,
    #[prost(float, tag = "12")]
    pub thcv: f32,
    #[prost(float, tag = "13")]
    pub thca: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CannibanoidScreenById {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerpenoidScreen {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub a_bisabolol: f32,
    #[prost(float, tag = "5")]
    pub a_humulene: f32,
    #[prost(float, tag = "6")]
    pub a_pinene: f32,
    #[prost(float, tag = "7")]
    pub a_terpinene: f32,
    #[prost(float, tag = "8")]
    pub b_caryophyllene: f32,
    #[prost(float, tag = "9")]
    pub b_myrcene: f32,
    #[prost(float, tag = "10")]
    pub b_pinene: f32,
    #[prost(float, tag = "11")]
    pub camphene: f32,
    #[prost(float, tag = "12")]
    pub caryophyllene_oxide: f32,
    #[prost(float, tag = "13")]
    pub delta_3_carene: f32,
    #[prost(float, tag = "14")]
    pub gamma_terpinene: f32,
    #[prost(float, tag = "15")]
    pub geraniol: f32,
    #[prost(float, tag = "16")]
    pub guaiol: f32,
    #[prost(float, tag = "17")]
    pub isopulegol: f32,
    #[prost(float, tag = "18")]
    pub linalool: f32,
    #[prost(float, tag = "19")]
    pub trans_nerolidol: f32,
    #[prost(float, tag = "20")]
    pub ocimene: f32,
    #[prost(float, tag = "21")]
    pub p_cymene: f32,
    #[prost(float, tag = "22")]
    pub eucalyptol: f32,
    #[prost(float, tag = "23")]
    pub terpinolene: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTerpenoidScreen {
    #[prost(string, tag = "1")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(float, tag = "3")]
    pub a_bisabolol: f32,
    #[prost(float, tag = "4")]
    pub a_humulene: f32,
    #[prost(float, tag = "5")]
    pub a_pinene: f32,
    #[prost(float, tag = "6")]
    pub a_terpinene: f32,
    #[prost(float, tag = "7")]
    pub b_caryophyllene: f32,
    #[prost(float, tag = "8")]
    pub b_myrcene: f32,
    #[prost(float, tag = "9")]
    pub b_pinene: f32,
    #[prost(float, tag = "10")]
    pub camphene: f32,
    #[prost(float, tag = "11")]
    pub caryophyllene_oxide: f32,
    #[prost(float, tag = "12")]
    pub delta_3_carene: f32,
    #[prost(float, tag = "13")]
    pub gamma_terpinene: f32,
    #[prost(float, tag = "14")]
    pub geraniol: f32,
    #[prost(float, tag = "15")]
    pub guaiol: f32,
    #[prost(float, tag = "16")]
    pub isopulegol: f32,
    #[prost(float, tag = "17")]
    pub linalool: f32,
    #[prost(float, tag = "18")]
    pub trans_nerolidol: f32,
    #[prost(float, tag = "19")]
    pub ocimene: f32,
    #[prost(float, tag = "20")]
    pub p_cymene: f32,
    #[prost(float, tag = "21")]
    pub eucalyptol: f32,
    #[prost(float, tag = "22")]
    pub terpinolene: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerpenoidScreenById {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestResults {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub cannibanoid_screen_id: i32,
    #[prost(int32, tag = "5")]
    pub terpenoid_screen_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTestResults {
    #[prost(string, tag = "1")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub cannibanoid_screen_id: i32,
    #[prost(int32, tag = "4")]
    pub terpenoid_screen_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignTestResultsRequest {
    #[prost(string, tag = "1")]
    pub cann_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub terp_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub grower_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub batch_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestResultById {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod grower_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GrowerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GrowerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GrowerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GrowerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GrowerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn create_cannibanoid_screen(
            &mut self,
            request: impl tonic::IntoRequest<super::NewCannibanoidScreen>,
        ) -> Result<tonic::Response<super::CannibanoidScreen>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/CreateCannibanoidScreen",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_cannibanoid_screen(
            &mut self,
            request: impl tonic::IntoRequest<super::CannibanoidScreenById>,
        ) -> Result<tonic::Response<super::CannibanoidScreen>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/GetCannibanoidScreen",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_terpenoid_screen(
            &mut self,
            request: impl tonic::IntoRequest<super::NewTerpenoidScreen>,
        ) -> Result<tonic::Response<super::TerpenoidScreen>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/CreateTerpenoidScreen",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_terpenoid_screen(
            &mut self,
            request: impl tonic::IntoRequest<super::TerpenoidScreenById>,
        ) -> Result<tonic::Response<super::TerpenoidScreen>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/GetTerpenoidScreen",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_test_results(
            &mut self,
            request: impl tonic::IntoRequest<super::NewTestResults>,
        ) -> Result<tonic::Response<super::TestResults>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/CreateTestResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn assign_test_results(
            &mut self,
            request: impl tonic::IntoRequest<super::AssignTestResultsRequest>,
        ) -> Result<tonic::Response<super::TestResults>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/AssignTestResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_test_results(
            &mut self,
            request: impl tonic::IntoRequest<super::TestResultById>,
        ) -> Result<tonic::Response<super::TestResults>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grower.Grower/GetTestResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod grower_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GrowerServer.
    #[async_trait]
    pub trait Grower: Send + Sync + 'static {
        async fn create_cannibanoid_screen(
            &self,
            request: tonic::Request<super::NewCannibanoidScreen>,
        ) -> Result<tonic::Response<super::CannibanoidScreen>, tonic::Status>;
        async fn get_cannibanoid_screen(
            &self,
            request: tonic::Request<super::CannibanoidScreenById>,
        ) -> Result<tonic::Response<super::CannibanoidScreen>, tonic::Status>;
        async fn create_terpenoid_screen(
            &self,
            request: tonic::Request<super::NewTerpenoidScreen>,
        ) -> Result<tonic::Response<super::TerpenoidScreen>, tonic::Status>;
        async fn get_terpenoid_screen(
            &self,
            request: tonic::Request<super::TerpenoidScreenById>,
        ) -> Result<tonic::Response<super::TerpenoidScreen>, tonic::Status>;
        async fn create_test_results(
            &self,
            request: tonic::Request<super::NewTestResults>,
        ) -> Result<tonic::Response<super::TestResults>, tonic::Status>;
        async fn assign_test_results(
            &self,
            request: tonic::Request<super::AssignTestResultsRequest>,
        ) -> Result<tonic::Response<super::TestResults>, tonic::Status>;
        async fn get_test_results(
            &self,
            request: tonic::Request<super::TestResultById>,
        ) -> Result<tonic::Response<super::TestResults>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GrowerServer<T: Grower> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Grower> GrowerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GrowerServer<T>
    where
        T: Grower,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grower.Grower/CreateCannibanoidScreen" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCannibanoidScreenSvc<T: Grower>(pub Arc<T>);
                    impl<
                        T: Grower,
                    > tonic::server::UnaryService<super::NewCannibanoidScreen>
                    for CreateCannibanoidScreenSvc<T> {
                        type Response = super::CannibanoidScreen;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewCannibanoidScreen>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_cannibanoid_screen(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateCannibanoidScreenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grower.Grower/GetCannibanoidScreen" => {
                    #[allow(non_camel_case_types)]
                    struct GetCannibanoidScreenSvc<T: Grower>(pub Arc<T>);
                    impl<
                        T: Grower,
                    > tonic::server::UnaryService<super::CannibanoidScreenById>
                    for GetCannibanoidScreenSvc<T> {
                        type Response = super::CannibanoidScreen;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CannibanoidScreenById>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_cannibanoid_screen(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCannibanoidScreenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grower.Grower/CreateTerpenoidScreen" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTerpenoidScreenSvc<T: Grower>(pub Arc<T>);
                    impl<
                        T: Grower,
                    > tonic::server::UnaryService<super::NewTerpenoidScreen>
                    for CreateTerpenoidScreenSvc<T> {
                        type Response = super::TerpenoidScreen;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewTerpenoidScreen>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_terpenoid_screen(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTerpenoidScreenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grower.Grower/GetTerpenoidScreen" => {
                    #[allow(non_camel_case_types)]
                    struct GetTerpenoidScreenSvc<T: Grower>(pub Arc<T>);
                    impl<
                        T: Grower,
                    > tonic::server::UnaryService<super::TerpenoidScreenById>
                    for GetTerpenoidScreenSvc<T> {
                        type Response = super::TerpenoidScreen;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TerpenoidScreenById>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_terpenoid_screen(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTerpenoidScreenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grower.Grower/CreateTestResults" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTestResultsSvc<T: Grower>(pub Arc<T>);
                    impl<T: Grower> tonic::server::UnaryService<super::NewTestResults>
                    for CreateTestResultsSvc<T> {
                        type Response = super::TestResults;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewTestResults>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_test_results(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTestResultsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grower.Grower/AssignTestResults" => {
                    #[allow(non_camel_case_types)]
                    struct AssignTestResultsSvc<T: Grower>(pub Arc<T>);
                    impl<
                        T: Grower,
                    > tonic::server::UnaryService<super::AssignTestResultsRequest>
                    for AssignTestResultsSvc<T> {
                        type Response = super::TestResults;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssignTestResultsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).assign_test_results(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssignTestResultsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grower.Grower/GetTestResults" => {
                    #[allow(non_camel_case_types)]
                    struct GetTestResultsSvc<T: Grower>(pub Arc<T>);
                    impl<T: Grower> tonic::server::UnaryService<super::TestResultById>
                    for GetTestResultsSvc<T> {
                        type Response = super::TestResults;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestResultById>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_test_results(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTestResultsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Grower> Clone for GrowerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Grower> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Grower> tonic::server::NamedService for GrowerServer<T> {
        const NAME: &'static str = "grower.Grower";
    }
}
