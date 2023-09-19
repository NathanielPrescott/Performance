use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::DynamicImage;
use jpeg_encoder::Encoder;
use serde::Deserialize;
use std::thread;
use std::time::Instant;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use imagestorage::image_storage_server::{ImageStorage, ImageStorageServer};
use imagestorage::{Image, ImageSize, Message, MessageIdentifier};

#[derive(Deserialize, Debug)]
enum Size {
    Small,
    Medium,
    Large,
    Original,
}

struct Images {
    small: Vec<u8>,
    medium: Vec<u8>,
    large: Vec<u8>,
    original: Vec<u8>,
}

pub mod imagestorage {
    include!("imagestorage.rs");
}

pub struct ImageStorageService {
    images: &'static Images,
}

#[tonic::async_trait]
impl ImageStorage for ImageStorageService {
    async fn get_image(&self, request: Request<ImageSize>) -> Result<Response<Image>, Status> {
        println!("Got a request: {:?}", request.into_inner().size);
        Ok(Response::new(Image {
            data: self.images.medium.clone(),
        }))
    }

    async fn get_message(
        &self,
        request: Request<MessageIdentifier>,
    ) -> Result<Response<Message>, Status> {
        println!("Got a request: {:?}", request);
        Ok(Response::new(Message {
            text: "Hello from the server!".into(),
        }))
    }
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
                println!("Encoding image {:?}...", &mut size);
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
}

#[get("/image/request/{size}")]
async fn image_request(size: web::Path<Size>, data: Data<&Images>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(match size.into_inner() {
            Size::Small => data.small.clone(),
            Size::Medium => data.medium.clone(),
            Size::Large => data.large.clone(),
            Size::Original => data.original.clone(),
        })
}

#[get("/message")]
async fn image_deliver() -> impl Responder {
    "Service is running and ready to deliver images"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let domain = "localhost";
    let rest_port = 8080;
    let grpc_port = 50051;

    println!("Starting servers...");
    println!("Listening on: http://{}:{}", domain, rest_port);
    println!("Listening on: http://{}:{}", domain, grpc_port);

    let images = Images::new();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ImageStorageServer::new(ImageStorageService { images }))
            .serve(
                ("[::1]:".to_owned() + grpc_port.to_string().as_str())
                    .parse()
                    .unwrap(),
            )
            .await
            .unwrap();
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(Data::new(images))
            .service(image_request)
            .service(image_deliver)
    })
    .bind((domain, rest_port))?
    .run()
    .await
}
