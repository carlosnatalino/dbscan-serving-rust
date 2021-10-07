use tonic::{transport::Server, Code, Request, Response, Status};

mod dbscanserving;

use dbscanserving::algorithm::{SymmetricMatrix, DBSCAN};
use dbscanserving::detector_server::{Detector, DetectorServer};
use dbscanserving::{DetectionRequest, DetectionResponse, Metric};

use actix_web::{error, middleware, rt, web, App, HttpResponse, HttpServer, Responder};
use std::process;
use std::thread;

#[derive(Debug, Default)]
pub struct MyDetector {}

#[tonic::async_trait]
impl Detector for MyDetector {
    async fn detect(
        &self,
        request: Request<DetectionRequest>,
    ) -> Result<Response<DetectionResponse>, Status> {
        // println!("gRPC request received: {:?}", request);

        // getting the detection request
        let detection_request = request.into_inner();

        // validating the dataset dimensions
        if detection_request.dimensions.len() != 2 {
            return Err(Status::new(
                Code::OutOfRange,
                format!("The declared dataset dimensions should be of length `2`! Instead, if is currently of length {}", detection_request.dimensions.len()),
            ));
        }
        // validating the number of samples within the dataset
        if detection_request.dimensions[0] != detection_request.samples.len() as i32 {
            return Err(Status::new(
                Code::OutOfRange,
                format!("The declared number of samples is `{}` but the received dataset contains `{}`!", detection_request.dimensions[0], detection_request.samples.len()),
            ));
        }

        match Metric::from_i32(detection_request.metric) {
            Some(Metric::Euclidean) => {
                let samples = detection_request.samples;

                let mut matrix = SymmetricMatrix::<f32>::new(samples.len());

                for (i, o1) in samples.iter().enumerate() {
                    // validating the number of features
                    if o1.features.len() as i32 != detection_request.dimensions[1] {
                        return Err(Status::new(
                            Code::OutOfRange,
                            format!("The feature length of sample {} does not match with the declared dimensions!", i+1),
                        ));
                    }
                    for (j, o2) in samples.iter().enumerate() {
                        if i < j {
                            // only need to compute the distance between a pair of samples once
                            let mut distance: f32 = 0.;
                            for (p1, p2) in o1.features.iter().zip(&o2.features) {
                                distance += (p1 - p2).powi(2);
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

                // creating an instance of DBSCAN
                let mut alg = DBSCAN::<f32>::new(
                    detection_request.eps,
                    detection_request.min_samples as usize,
                );

                // performing the clustering
                let clusters = alg.perform_clustering(&matrix);

                // println!("\nClusters: {:?}", clusters);

                // converting the indices to the format expected by the response message
                let indices = clusters
                    .iter()
                    .map(|&x| match x {
                        Some(i) => i as i32,
                        None => -1i32,
                    })
                    .collect();

                Ok(Response::new(DetectionResponse {
                    cluster_indices: indices,
                }))
            }
            None => Err(Status::new(
                Code::InvalidArgument,
                "The distance function was not correctly set! Check again the protobuffer.",
            )),
        }
    }
}

async fn detect(detection_request: web::Json<DetectionRequest>) -> impl Responder {
    // println!("REST request received");

    if detection_request.dimensions.len() != 2 {
        let reply = DetectionResponse {
            cluster_indices: vec![10],
        };
        println!("Error!");
        // TODO: improve error reply
        return web::Json(reply);
    }
    // validating the number of samples within the dataset
    if detection_request.dimensions[0] != detection_request.samples.len() as i32 {
        let reply = DetectionResponse {
            cluster_indices: vec![10],
        };
        println!("Error!");
        // TODO: improve error reply
        return web::Json(reply);
    }

    match Metric::from_i32(detection_request.metric) {
        Some(Metric::Euclidean) => {
            let samples = &detection_request.samples;

            // println!("Processing {} samples", samples.len());

            let mut matrix = SymmetricMatrix::<f32>::new(samples.len());

            for (i, o1) in samples.iter().enumerate() {
                if o1.features.len() as i32 != detection_request.dimensions[1] {
                    let reply = DetectionResponse {
                        cluster_indices: vec![10],
                    };
                    println!("Error!");
                    // TODO: improve error reply
                    return web::Json(reply);
                }
                for (j, o2) in samples.iter().enumerate() {
                    if i < j {
                        // println!("{} -> {}", o1.id, o2.id);
                        let mut distance: f32 = 0.;
                        for (p1, p2) in o1.features.iter().zip(&o2.features) {
                            distance += (p1 - p2).powi(2);
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

            let mut alg = DBSCAN::<f32>::new(
                detection_request.eps,
                detection_request.min_samples as usize,
            );

            let clusters = alg.perform_clustering(&matrix);

            // println!("\nClusters: {:?}", clusters);

            let indices = clusters
                .iter()
                .map(|&x| match x {
                    Some(i) => i as i32,
                    None => -1i32,
                })
                .collect();

            let reply = DetectionResponse {
                cluster_indices: indices,
            };
            println!("Success!");
            web::Json(reply)
        }
        None => {
            let reply = DetectionResponse {
                cluster_indices: vec![10],
            };
            println!("Error!");
            // TODO: improve error reply
            web::Json(reply)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:5051".parse()?; // use this for IPv6 -- may have conflicts with Docker
    let addr = "0.0.0.0:5051".parse()?;
    let detector = MyDetector::default();

    let _handler = thread::spawn(|| {
        let mut _sys = rt::System::new("rest");
        let rest_port = 5052;
        println!("Starting to serve REST on {}", format!("0.0.0.0:{}", rest_port));

        HttpServer::new(|| {
            App::new()
                // enable logger
                .wrap(middleware::Logger::default())
                .service(web::resource("/").to(|| async { "Call the /detect endpoint!" }))
                .service(
                    web::resource("/detect")
                        // enabling the server to receive large requests
                        .app_data(web::JsonConfig::default().limit(409600).error_handler(
                            |err, _req| {
                                // create custom error response
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                .into()
                            },
                        ))
                        .to(detect),
                )
        })
        .bind(format!("0.0.0.0:{}", rest_port))
        .unwrap()
        .run();
        thread::park();
    });

    ctrlc::set_handler(|| {
        println!("Stopping...");
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("Starting to serve GRPC on {}", addr);
    Server::builder()
        .add_service(DetectorServer::new(detector))
        .serve(addr)
        .await?;
    Ok(())
}
