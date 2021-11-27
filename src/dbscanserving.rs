#[derive(Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct Sample {
    #[prost(float, repeated, tag = "1")]
    pub features: ::prost::alloc::vec::Vec<f32>,
}

#[derive(Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct DetectionRequest {
    #[prost(float, tag = "1")]
    pub eps: f32,
    #[prost(int32, tag = "2")]
    pub min_samples: i32,
    #[prost(enumeration = "Metric", tag = "3")]
    pub metric: i32,
    #[prost(int32, tag = "4")]
    pub num_samples: i32,
    #[prost(int32, tag = "5")]
    pub num_features: i32,
    #[prost(message, repeated, tag = "6")]
    pub samples: ::prost::alloc::vec::Vec<Sample>,
    #[prost(int32, optional, tag = "7")]
    pub identifier: ::core::option::Option<i32>,
}

#[derive(Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct DetectionResponse {
    #[prost(int32, repeated, tag = "1")]
    pub cluster_indices: ::prost::alloc::vec::Vec<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Metric {
    Euclidean = 0,
}

/// Generated client implementations.
pub mod detector_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    pub struct DetectorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for DetectorClient<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                DetectorClient {
                    inner: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "DetectorClient");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "inner",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for DetectorClient<T> {
        #[inline]
        fn clone(&self) -> DetectorClient<T> {
            match *self {
                DetectorClient {
                    inner: ref __self_0_0,
                } => DetectorClient {
                    inner: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl DetectorClient<tonic::transport::Channel> {
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
    impl<T> DetectorClient<T>
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
        ) -> DetectorClient<InterceptedService<T, F>>
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
            DetectorClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn detect(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectionRequest>,
        ) -> Result<tonic::Response<super::DetectionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(tonic::Code::Unknown, format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/dbscanserving.Detector/Detect");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod detector_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DetectorServer.
    pub trait Detector: Send + Sync + 'static {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn detect<'life0, 'async_trait>(
            &'life0 self,
            request: tonic::Request<super::DetectionRequest>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            tonic::Response<super::DetectionResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
    pub struct DetectorServer<T: Detector> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Detector> ::core::fmt::Debug for DetectorServer<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                DetectorServer {
                    inner: ref __self_0_0,
                    accept_compression_encodings: ref __self_0_1,
                    send_compression_encodings: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "DetectorServer");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "inner",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "accept_compression_encodings",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "send_compression_encodings",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Detector> DetectorServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DetectorServer<T>
    where
        T: Detector,
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
                "/dbscanserving.Detector/Detect" => {
                    #[allow(non_camel_case_types)]
                    struct DetectSvc<T: Detector>(pub Arc<T>);
                    impl<T: Detector> tonic::server::UnaryService<super::DetectionRequest> for DetectSvc<T> {
                        type Response = super::DetectionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DetectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).detect(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DetectSvc(inner);
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
    impl<T: Detector> Clone for DetectorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Detector> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Detector> tonic::transport::NamedService for DetectorServer<T> {
        const NAME: &'static str = "dbscanserving.Detector";
    }
}