use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{App, HttpServer, Responder};
use image::DynamicImage;
use jpeg_encoder::Encoder;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::thread;
use std::time::Instant;
use tonic::transport::Server;
use tonic::IntoRequest;

use crate::rest::{image_deliver, image_request};

mod rest;
mod streaming;

#[derive(Serialize, Deserialize, Debug)]
enum Size {
    Small,
    Medium,
    Large,
    Original,
}

#[derive(Serialize, Debug)]
pub struct ImageStorageService {
    images: &'static Images,
}

#[derive(Serialize, Debug)]
struct Images {
    small: Vec<u8>,
    medium: Vec<u8>,
    large: Vec<u8>,
    original: Vec<u8>,
}

impl Images {
    fn new() -> &'static Images {
        let start = Instant::now();

        let path = "src/files/image.JPG";
        let image = image::open(path).unwrap_or_default();

        let sizes = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        let mut handles = Vec::new();
        let mut encoder_quality = 25;
        for mut size in sizes {
            let image = image.clone();

            let handle = thread::spawn(move || {
                println!("Encoding image quality %{:?}...", &mut encoder_quality);
                Self::image_encode(image, &mut size, encoder_quality);
                size
            });

            encoder_quality += 25;

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().unwrap());
        }

        let (small, medium, large, original) = (
            results.get(0).unwrap(),
            results.get(1).unwrap(),
            results.get(2).unwrap(),
            results.get(3).unwrap(),
        );

        let duration = start.elapsed();
        println!("Images loaded in: {:?}", duration);

        Box::leak(Box::new(Images {
            small: small.clone(),
            medium: medium.clone(),
            large: large.clone(),
            original: original.clone(),
        }))
    }

    fn image_encode(image: DynamicImage, mut size: &mut Vec<u8>, quality: u8) {
        let encoder = Encoder::new(&mut size, quality);

        encoder
            .encode(
                image.as_bytes(),
                image.width() as u16,
                image.height() as u16,
                jpeg_encoder::ColorType::Rgb,
            )
            .unwrap();
    }

    fn from_string(size: &str) -> Result<Size, &'static str> {
        match size {
            "Small" => Ok(Size::Small),
            "Medium" => Ok(Size::Medium),
            "Large" => Ok(Size::Large),
            "Original" => Ok(Size::Original),
            _ => Err("Invalid size name, please check spelling."),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let domain = "localhost";
    let rest_port = 8080;
    let rst2_port = 8081;
    let webs_port = 8082;
    let grpc_port = 50051;

    println!("Starting servers...");
    println!("Listening on: http://{}:{}", domain, rest_port);
    println!("Listening on: http://{}:{}", domain, rest_port);
    println!("Listening on: http://{}:{}", domain, webs_port);
    println!("Listening on: http://{}:{}", domain, grpc_port);

    let images = Images::new();

    tokio::spawn(async move {
        let service = ImageStorageService::new(ImageStorageService { images });
        let address = "[::1]:".to_owned() + grpc_port.to_string().as_str();

        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(service))
            .serve(address.parse().unwrap())
            .await
            .unwrap();
    });

    let mut builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("src/files/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("src/files/cert.pem")
        .unwrap();

    HttpServer::new(|| App::new().service(image_deliver).service(image_request))
        .bind_openssl((domain, rst2_port), builder)?
        .run()
        .await

    // HttpServer::new(move || {
    //     let cors = Cors::permissive();
    //
    //     App::new()
    //         .wrap(cors)
    //         .app_data(Data::new(images))
    //         .service(web::resource("/ws_message").to(ws_message))
    // })
    // .bind((domain, webs_port)).unwrap()
    // .run()
    // .await;

    // HttpServer::new(move || {
    //     let cors = Cors::permissive();
    //
    //     App::new()
    //         .wrap(cors)
    //         .app_data(Data::new(images))
    //         .service(image_request)
    //         .service(image_deliver)
    // })
    // .bind((domain, rest_port))?
    // .run()
    // .await
}
