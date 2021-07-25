use tonic::{transport::Server, Request, Response, Status};

use dbscanserving::detector_server::{Detector, DetectorServer};
use dbscanserving::{DetectionRequest, DetectionResponse};
use ndarray::Array;
use linfa_clustering::{Dbscan};
use linfa::traits::Transformer;

use dbscanserving::Sample;

pub mod dbscanserving {
    tonic::include_proto!("dbscanserving"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyDetector{}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[tonic::async_trait]
impl Detector for MyDetector {

    async fn detect(&self, request: Request<DetectionRequest>) -> Result<Response<DetectionResponse>, Status> {

        println!("REQUEST: {:?}", request);

        let detection_request = request.into_inner();

        let samples = detection_request.samples;

        println!("Samples: {:?}\n", samples);

        // let observations = Array::from_shape_vec((1, samples.len()), samples).unwrap();

        // println!("Observations: {:?}\n", observations);

        let obs = samples.into_iter()
            .map(|sample| sample.features)
            .collect::<Vec<_>>();

        println!("Obs: {:?}\n", obs);

        let width = obs[1].len();
        // ndarray::Array2::from_shape_vec

        let observations = Array::from_shape_vec((obs.len(), obs[0].len()), obs).unwrap();

        // observations.into_shape((obs.len(), width));

        let clusters = Dbscan::params(detection_request.model_spec.unwrap().min_samples as usize)
            .tolerance(detection_request.model_spec.unwrap().eps)
            .transform(&observations);
        
        println!("Clusters: {:?}\n\n\n\n", clusters);

        let reply = DetectionResponse {
            cluster_indices: vec![30],
        };

        Ok(Response::new(reply))
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
