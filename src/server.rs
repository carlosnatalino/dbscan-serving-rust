use std::process;
use std::thread;

use tonic::transport::Server;

use actix_web::{error, middleware, rt, web, App, HttpResponse, HttpServer};

mod metrics;

mod handlers;
use handlers::{MyDetector, healthz, prometheus_metrics, detect};

mod dbscanserving;
use dbscanserving::detector_server::DetectorServer;

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
                .service(web::resource("/metrics").to(prometheus_metrics))
                .service(web::resource("/healthz").to(healthz))
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
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DetectorServer<MyDetector>>()
        .await;
    

    Server::builder()
        .add_service(health_service)
        .add_service(DetectorServer::new(detector))
        .serve(addr)
        .await?;
    Ok(())
}
