mod dbscanserving;
use dbscanserving::detector_client::DetectorClient;
use dbscanserving::{DetectionRequest, Metric, Sample};

use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let mut client = DetectorClient::connect("http://[::1]:50051").await?;

    let now = std::time::Instant::now();

    // running 200 times to get a sense of performance
    for _ in 1i32..200 {
        let mut samples: Vec<Sample> = Vec::new();
        let dim = 100;

        // generating 200 samples for the first cluster
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
        for _ in 0..100 {
            let mut sample1 = Sample::default();
            let mut vec = Vec::<f32>::new();
            for _ in 0..dim {
                vec.push(rng.gen_range(50.0..60.0));
            }
            sample1.features = vec;
            samples.push(sample1);
        }

        // generating 10 anomalous samples
        for _ in 0..10 {
            let mut sample1 = Sample::default();
            let mut vec = Vec::<f32>::new();
            for _ in 0..dim {
                vec.push(rng.gen_range(100000.0..20000000000.0));
            }
            sample1.features = vec;
            samples.push(sample1);
        }

        let request = tonic::Request::new(DetectionRequest {
            eps: 100.5,
            min_samples: 50,
            metric: Metric::Euclidean as i32,
            dimensions: vec![samples.len() as i32, dim],
            samples,
        });

        // sending the request
        client.detect(request).await?;
        // println!("RESPONSE={:?}", response);
        // println!("Length: {}", response.into_inner().cluster_indices.len());
    }

    println!("Time: {}", now.elapsed().as_millis());

    Ok(())
}
