mod dbscanserving;
use dbscanserving::detector_client::DetectorClient;
use dbscanserving::{DetectionRequest, Metric, Sample};
use std::{thread, time::Duration};

use rand::Rng;

use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let client_ip = "10.108.250.232";
    let num_reqs = 10000;

    let mut grpc_client = DetectorClient::connect(format!("http://{}:5051", client_ip)).await?;

    // let request_url = "http://localhost:5052/detect";
    // let rest_client = Client::new();

    let mut sum_grpc: u128 = 0;
    // let mut sum_rest: u128 = 0;

    println!("Starting the gRPC test!");

    // running 200 times to get a sense of performance
    let dim = 200;
    for ida in 1i32..num_reqs {
        let mut samples: Vec<Sample> = Vec::new();

        // generating 200 samples for the first cluster
        // [30, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 400, 500]
        for _ in 0..200 {
            let mut _sample = Sample::default();
            let mut vec = Vec::<f32>::new();
            for _ in 0..dim {
                vec.push(rng.gen_range(0.0..10.0));
            }
            _sample.features = vec;
            samples.push(_sample);
        }

        // generating 100 samples for the second cluster
        // for _ in 0..100 {
        //     let mut sample1 = Sample::default();
        //     let mut vec = Vec::<f32>::new();
        //     for _ in 0..dim {
        //         vec.push(rng.gen_range(50.0..60.0));
        //     }
        //     sample1.features = vec;
        //     samples.push(sample1);
        // }

        // generating 10 anomalous samples
        // for _ in 0..10 {
        //     let mut sample1 = Sample::default();
        //     let mut vec = Vec::<f32>::new();
        //     for _ in 0..dim {
        //         vec.push(rng.gen_range(100000.0..20000000000.0));
        //     }
        //     sample1.features = vec;
        //     samples.push(sample1);
        // }

        let detection_request = DetectionRequest {
            eps: 100.5,
            min_samples: 50,
            metric: Metric::Euclidean as i32,
            num_samples: samples.len() as i32,
            num_features: dim,
            samples,
            identifier: Some(ida),
        };

        // let now = std::time::Instant::now();
        // let _response = rest_client
        //     .post(request_url)
        //     .json(&detection_request)
        //     .send()
        //     .await?;
        // sum_rest += now.elapsed().as_millis();

        let request = tonic::Request::new(detection_request);

        // sending the request
        let now = std::time::Instant::now();
        let response = grpc_client.detect(request).await?;
        // println!("response: {:?}", response);
        sum_grpc += now.elapsed().as_millis();
        // thread::sleep(Duration::from_millis(100));

        println!("RESPONSE={:?}", ida);
        // println!("Length: {}", response.into_inner().cluster_indices.len());
    }

    println!("Avg. time gRPC: {}", sum_grpc as f64 / num_reqs as f64);
    // println!("Avg. time REST: {}", sum_rest as f64 / 200.);

    Ok(())
}
