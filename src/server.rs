use std::env;
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
    dotenv::from_filename(".env.local").ok();

    // if let Some(env::var("BIND_ADDRESS")) = bind_address;

    let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let grpc_port = env::var("GRPC_API_PORT").unwrap_or("8500".to_string()).parse::<i32>().unwrap_or_else(|_| panic!("The value `{}` is not a valid port!", env::var("GRPC_PORT").unwrap()));
    let rest_port = env::var("REST_API_PORT").unwrap_or("8501".to_string()).parse::<i32>().unwrap_or_else(|_| panic!("The value `{}` is not a valid port!", env::var("GRPC_PORT").unwrap()));
    let metrics_port = env::var("METRICS_PORT").unwrap_or("9192".to_string()).parse::<i32>().unwrap_or_else(|_| panic!("The value `{}` is not a valid port!", env::var("METRICS_PORT").unwrap()));

    // let addr = "[::1]:5051".parse()?; // use this for IPv6 -- may have conflicts with Docker
    let addr = format!("{}:{}", bind_address, grpc_port).parse()?;
    let detector = MyDetector::default();

    if rest_port > 0 {
        let _handler = thread::spawn(move|| {
            let mut _sys = rt::System::new("rest");
            let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
            
            println!("Starting to serve REST on {}", format!("{}:{}", bind_address, rest_port));

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
            .bind(format!("{}:{}", bind_address, rest_port))
            .unwrap()
            .run();
            thread::park();
        });
    }

    if metrics_port > 0 {
        let _handler = thread::spawn(move|| {
            let mut _sys = rt::System::new("rest");
            let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
            
            println!("Starting to serve metrics on {}", format!("{}:{}", bind_address, metrics_port));

            HttpServer::new(|| {
                App::new()
                    // enable logger
                    .wrap(middleware::Logger::default())
                    .service(web::resource("/metrics").to(prometheus_metrics))
            })
            .bind(format!("{}:{}", bind_address, metrics_port))
            .unwrap()
            .run();
            thread::park();
        });
    }

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
