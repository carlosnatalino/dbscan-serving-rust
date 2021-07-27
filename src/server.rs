use tonic::{transport::Server, Request, Response, Status, Code};

mod dbscanserving;

use dbscanserving::detector_server::{Detector, DetectorServer};
use dbscanserving::{DetectionRequest, DetectionResponse, Metric};

use dbscanserving::algorithm::{DBSCAN, SymmetricMatrix};

#[derive(Debug, Default)]
pub struct MyDetector{}

#[tonic::async_trait]
impl Detector for MyDetector {

    async fn detect(&self, request: Request<DetectionRequest>) -> Result<Response<DetectionResponse>, Status> {

        // println!("REQUEST: {:?}", request);

        let detection_request = request.into_inner();

        match Metric::from_i32(detection_request.metric) {
            Some(Metric::Euclidean) => {
                let samples = detection_request.samples;

                let mut matrix = SymmetricMatrix::<f32>::new(samples.len());

                for (i, o1) in samples.iter().enumerate() {
                    for (j, o2) in samples.iter().enumerate() {
                        if i < j {
                            // println!("{} -> {}", o1.id, o2.id);
                            if o1.features.len() != o2.features.len() {
                                return Err(Status::new(Code::OutOfRange, "The feature length among the samples do not match!"));
                            }
                            let mut distance: f32 = 0.;
                            for (p1, p2) in o1.features.iter().zip(&o2.features) {
                                distance = distance + (p1 - p2).powi(2);
                                // println!("P1: {}\tP2: {}\tDistance: {}", p1, p2, distance);
                            }
                            // println!("Distance: {}", distance.sqrt());
                            matrix.set(i, j, distance.sqrt());
                            matrix.set(j, i, distance.sqrt());
                            // sum_matrix += distance.sqrt();
                        }
                    }
                }

                // println!("Average distance: {}", sum_matrix / (samples.len() * samples.len()) as f32);

                let mut alg = DBSCAN::<f32>::new(detection_request.eps, detection_request.min_samples as usize);

                let clusters = alg.perform_clustering(&matrix);

                // println!("\nClusters: {:?}", clusters);

                let indices = clusters.into_iter()
                    .map(|&x| {
                        match x {
                            Some(i) => i as i32,
                            None => -1i32,
                        }
                    })
                    .collect();

                let reply = DetectionResponse {
                    cluster_indices: indices,
                };

                Ok(Response::new(reply))
            }
            None => {
                Err(Status::new(Code::InvalidArgument, "The distance function was not correctly set!"))
            }
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let detector = MyDetector::default();

    println!("Starting to serve...");
    Server::builder()
        .add_service(DetectorServer::new(detector))
        .serve(addr)
        .await?;
    Ok(())
}
