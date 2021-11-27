use prometheus::{Histogram, IntCounter};
use prometheus::{register_int_counter, register_histogram};
use lazy_static::lazy_static;

const HTTP_RESPONSE_TIME_CUSTOM_BUCKETS: &[f64; 14] = &[
    0.0005, 0.001, 0.0015, 0.002, 0.0025, 0.003, 0.0035, 0.004, 0.0045, 0.005, 0.01, 0.05, 0.1, 1.0,
];

lazy_static! {
    pub static ref INCOMING_REQUESTS_GRPC: IntCounter =
        register_int_counter!("incoming_requests_grpc", "Incoming Requests gRPC")
        .expect("metric cannot be created");
    pub static ref INCOMING_REQUESTS_REST: IntCounter =
        register_int_counter!("incoming_requests_rest", "Incoming Requests REST")
        .expect("metric cannot be created");
    pub static ref RESPONSE_TIME_GRPC: Histogram = 
        register_histogram!("response_time_grpc", "Response times gRPC", HTTP_RESPONSE_TIME_CUSTOM_BUCKETS.to_vec())
        .expect("metric cannot be created");
    pub static ref RESPONSE_TIME_REST: Histogram = 
        register_histogram!("response_time_rest", "Response times REST", HTTP_RESPONSE_TIME_CUSTOM_BUCKETS.to_vec())
        .expect("metric cannot be created");
}
