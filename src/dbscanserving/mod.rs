#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sample {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(float, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectionRequest {
    #[prost(float, tag = "1")]
    pub eps: f32,
    #[prost(int32, tag = "2")]
    pub min_samples: i32,
    #[prost(enumeration = "Metric", tag = "3")]
    pub metric: i32,
    #[prost(int32, repeated, tag = "4")]
    pub dimensions: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "5")]
    pub samples: ::prost::alloc::vec::Vec<Sample>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectionResponse {
    #[prost(int32, repeated, tag = "1")]
    pub cluster_indices: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Metric {
    Euclidean = 0,
}

pub mod detector_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DetectorClient<T> {
        inner: tonic::client::Grpc<T>,
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
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
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/dbscanserving.Detector/Detect");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}

pub mod detector_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;

    /// Generated trait containing gRPC methods that should be implemented for
    /// use with [DetectorServer].
    #[async_trait]
    pub trait Detector: Send + Sync + 'static {
        async fn detect(
            &self,
            request: tonic::Request<super::DetectionRequest>,
        ) -> Result<tonic::Response<super::DetectionResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DetectorServer<T: Detector> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
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

pub mod algorithm {

    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct SymmetricMatrix<T> {
        size: usize,
        data: Vec<T>,
    }

    impl<T> SymmetricMatrix<T>
    where
        T: Default + Copy,
    {
        pub fn new(size: usize) -> Self {
            SymmetricMatrix {
                size,
                data: vec![T::default(); (size + 1) * size / 2],
            }
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn get(&self, row: usize, col: usize) -> T {
            let index = self.index_for(row, col);
            self.data[index]
        }
        pub fn set(&mut self, row: usize, col: usize, value: T) {
            let index = self.index_for(row, col);
            self.data[index] = value;
        }
        fn index_for(&self, row: usize, col: usize) -> usize {
            if col > row {
                col * (col + 1) / 2 + row
            } else {
                row * (row + 1) / 2 + col
            }
        }
    }

    #[derive(Debug)]
    #[allow(clippy::upper_case_acronyms)]
    pub struct DBSCAN<T> {
        eps: T,
        min_points: usize,
        clusters: Vec<Option<usize>>,
        visited: Vec<bool>,
        current_cluster: usize,
    }

    impl<T> DBSCAN<T>
    where
        T: Default + Copy + PartialOrd,
    {
        pub fn new(eps: T, min_points: usize) -> Self {
            DBSCAN {
                eps,
                min_points,
                clusters: Vec::new(),
                visited: Vec::new(),
                current_cluster: 0,
            }
        }

        pub fn perform_clustering(&mut self, matrix: &SymmetricMatrix<T>) -> &Vec<Option<usize>> {
            self.clusters = vec![None; matrix.size()];
            self.visited = vec![false; matrix.size()];
            self.current_cluster = 0;

            for point in 0..matrix.size() {
                if self.visited[point] {
                    continue;
                }
                self.visited[point] = true;
                let neighbors = self.region_query(matrix, point);
                if neighbors.len() >= self.min_points {
                    self.expand_cluster(matrix, point, neighbors);
                    self.current_cluster += 1;
                }
            }

            self.clusters.as_ref()
        }

        fn region_query(&self, matrix: &SymmetricMatrix<T>, point: usize) -> VecDeque<usize> {
            let mut neighbors = VecDeque::new();
            for other_point in 0..matrix.size() {
                let dist = matrix.get(point, other_point);
                if dist <= self.eps {
                    neighbors.push_back(other_point);
                }
            }
            neighbors
        }

        fn expand_cluster(
            &mut self,
            matrix: &SymmetricMatrix<T>,
            point: usize,
            mut neighbors: VecDeque<usize>,
        ) {
            self.clusters[point] = Some(self.current_cluster);

            while let Some(other_point) = neighbors.pop_front() {
                if !self.visited[other_point] {
                    self.visited[other_point] = true;
                    let mut other_neighbors = self.region_query(matrix, other_point);
                    if other_neighbors.len() >= self.min_points {
                        neighbors.append(&mut other_neighbors);
                    }
                }
                if self.clusters[other_point].is_none() {
                    self.clusters[other_point] = Some(self.current_cluster);
                }
            }
        }
    }
}
