// COPY PASTE FROM TARGET BECAUSE INTELLIJ DOES NOT SUPPORT TARGET FILE WATCH.
/// The request message containing the user's name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestVoteArgs {
    #[prost(int32, tag = "1")]
    pub term: i32,
    #[prost(int32, tag = "2")]
    pub candidate_id: i32,
}
/// The response message containing the greetings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestVoteReply {
    #[prost(int32, tag = "1")]
    pub term: i32,
    #[prost(bool, tag = "2")]
    pub vote_granted: bool,
}
#[doc = r" Generated client implementations."]
pub mod rpc_proxy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The greeting service definition."]
    #[derive(Debug, Clone)]
    pub struct RpcProxyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RpcProxyClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: std::convert::TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RpcProxyClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::ResponseBody: Body + Send + Sync + 'static,
            T::Error: Into<StdError>,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RpcProxyClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RpcProxyClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Sends a greeting"]
        pub async fn request_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestVoteArgs>,
        ) -> Result<tonic::Response<super::RequestVoteReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ruftp.RPCProxy/RequestVote");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod rpc_proxy_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RpcProxyServer."]
    #[async_trait]
    pub trait RpcProxy: Send + Sync + 'static {
        #[doc = " Sends a greeting"]
        async fn request_vote(
            &self,
            request: tonic::Request<super::RequestVoteArgs>,
        ) -> Result<tonic::Response<super::RequestVoteReply>, tonic::Status>;
    }
    #[doc = " The greeting service definition."]
    #[derive(Debug)]
    pub struct RpcProxyServer<T: RpcProxy> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RpcProxy> RpcProxyServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RpcProxyServer<T>
        where
            T: RpcProxy,
            B: Body + Send + Sync + 'static,
            B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ruftp.RPCProxy/RequestVote" => {
                    #[allow(non_camel_case_types)]
                    struct RequestVoteSvc<T: RpcProxy>(pub Arc<T>);
                    impl<T: RpcProxy> tonic::server::UnaryService<super::RequestVoteArgs> for RequestVoteSvc<T> {
                        type Response = super::RequestVoteReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestVoteArgs>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).request_vote(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RequestVoteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RpcProxy> Clone for RpcProxyServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RpcProxy> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RpcProxy> tonic::transport::NamedService for RpcProxyServer<T> {
        const NAME: &'static str = "ruftp.RPCProxy";
    }
}
