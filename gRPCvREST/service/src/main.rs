use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::DynamicImage;
use jpeg_encoder::Encoder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::pin::Pin;
use std::thread;
use std::time::Instant;
use tokio_stream::Stream;
use tonic::transport::Server;
use tonic::{IntoRequest, Request, Response, Status};

use imagestorage::image_storage_server::{ImageStorage, ImageStorageServer};
use imagestorage::{Image, MessageIdentifier, Statement};

mod imagestorage;

#[derive(Serialize, Deserialize, Debug)]
enum Size {
    Small,
    Medium,
    Large,
    Original,
}

#[derive(Serialize, Debug)]
struct Images {
    small: Vec<u8>,
    medium: Vec<u8>,
    large: Vec<u8>,
    original: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct ImageStorageService {
    images: &'static Images,
}

#[tonic::async_trait]
impl ImageStorage for ImageStorageService {
    type GetImageStream = Pin<Box<dyn Stream<Item = Result<Image, Status>> + Send + Sync>>;

    async fn get_image(
        &self,
        request: Request<imagestorage::Size>,
    ) -> Result<Response<Self::GetImageStream>, Status> {
        let size = Images::from_string(request.into_inner().size.as_str())
            .into_request()
            .into_inner()
            .map_err(|e| Status::invalid_argument(e))?;

        let output_stream = tokio_stream::iter(vec![Ok(Image {
            image: match size {
                Size::Small => self.images.small.clone(),
                Size::Medium => self.images.medium.clone(),
                Size::Large => self.images.large.clone(),
                Size::Original => self.images.original.clone(),
            },
        })]);

        Ok(Response::new(
            Box::pin(output_stream) as Self::GetImageStream
        ))
    }

    async fn get_message(
        &self,
        _request: Request<MessageIdentifier>,
    ) -> Result<Response<Statement>, Status> {
        Ok(Response::new(Statement {
            text: "Service is running and ready to deliver images".to_string(),
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
    println!("REST listening on: http://{}:{}", domain, rest_port);
    println!("gRPC listening on: http://{}:{}", domain, grpc_port);

    let images = Images::new();

    tokio::spawn(async move {
        let service = ImageStorageServer::new(ImageStorageService { images })
            .max_decoding_message_size(1024 * 1024 * 50);
        let address = "[::1]:".to_owned() + grpc_port.to_string().as_str();

        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(service))
            .serve(address.parse().unwrap())
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
