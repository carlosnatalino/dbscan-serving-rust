mod dbscanserving;
use dbscanserving::detector_client::DetectorClient;
use dbscanserving::{Metric, Sample, DetectionRequest};

use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let mut client = DetectorClient::connect("http://[::1]:50051").await?;

    let now = std::time::Instant::now();

    // let response = client.detect(request).await?;

    for _ in 1i32..200 {
        let mut samples: Vec<Sample> = Vec::new();

        for _ in 0..200 {
            let mut sample1 = Sample::default();
            sample1.id = 1;
            let mut vec = Vec::<f32>::new();
            for _ in 0..100 {
                vec.push(rng.gen_range(0.0..10.0));
            }
            sample1.features = vec;
            samples.push(sample1);
        }

        for _ in 0..100 {
            let mut sample1 = Sample::default();
            sample1.id = 1;
            let mut vec = Vec::<f32>::new();
            for _ in 0..100 {
                vec.push(rng.gen_range(50.0..60.0));
            }
            sample1.features = vec;
            samples.push(sample1);
        }

        for _ in 0..10 {
            let mut sample1 = Sample::default();
            sample1.id = 1;
            let mut vec = Vec::<f32>::new();
            for _ in 0..100 {
                vec.push(rng.gen_range(100000.0..20000000000.0));
            }
            sample1.features = vec;
            samples.push(sample1);
        }

        let request = tonic::Request::new(DetectionRequest {
            eps: 100.5,
            min_samples: 50,
            metric: Metric::Euclidean as i32,
            dimensions: vec![2, 30],
            samples: samples,
        });
        let response = client.detect(request).await?;
        // println!("RESPONSE={:?}", response);
        // println!("Length: {}", response.into_inner().cluster_indices.len());
    }

    println!("Time: {}", now.elapsed().as_millis());

    Ok(())

}
