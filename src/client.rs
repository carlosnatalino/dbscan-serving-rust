use dbscanserving::detector_client::DetectorClient;
use dbscanserving::{Metric, ModelSpec, Sample, DetectionRequest};


pub mod dbscanserving {
    tonic::include_proto!("dbscanserving");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DetectorClient::connect("http://[::1]:50051").await?;

    let mut model_spec = ModelSpec::default();
    model_spec.eps = 3.5;
    model_spec.min_samples = 30;
    model_spec.metric = Metric::Euclidean as i32;

    let mut samples: Vec<Sample> = Vec::new();
    let mut sample1 = Sample::default();
    sample1.features = vec![10., 11., 12.5, 45.4];
    samples.push(sample1);

    let mut sample2 = Sample::default();
    sample2.features = vec![10., 11., 12.5, 45.4];
    samples.push(sample2);

    let request = tonic::Request::new(DetectionRequest {
        model_spec: Some(model_spec),
        dimensions: vec![2, 30],
        samples: samples,
    });

    

    let now = std::time::Instant::now();

    let response = client.detect(request).await?;

    // for i in 1..20 {
    //     response = client.detect(request).await?;
    // }

    println!("RESPONSE={:?}", response);

    println!("Time: {}", now.elapsed().as_secs());

    Ok(())

}
