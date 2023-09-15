use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::DynamicImage;
use jpeg_encoder::Encoder;
use serde::Deserialize;
use std::thread;
use std::time::Instant;

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

#[get("/image/deliver")]
async fn image_deliver() -> impl Responder {
    "Service is running and ready to deliver images"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let localhost = "localhost";
    let port = 8080;

    println!("Starting server...");
    println!("Listening on: http://{}:{}", localhost, port);

    let images = Images::new();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(Data::new(images))
            .service(image_request)
            .service(image_deliver)
    })
    .bind((localhost, port))?
    .run()
    .await
}
