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
    ExtraLarge,
}

struct Images {
    small: Vec<u8>,
    medium: Vec<u8>,
    large: Vec<u8>,
    extra_large: Vec<u8>,
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

            if encoder_quality > 75 {
                encoder_quality = 90;
            }

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().unwrap());
        }

        let (small, medium, large, extra_large) = (
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
            extra_large: extra_large.clone(),
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
    let output = match size.into_inner() {
        Size::Small => data.small.clone(),
        Size::Medium => data.medium.clone(),
        Size::Large => data.large.clone(),
        Size::ExtraLarge => data.extra_large.clone(),
    };

    HttpResponse::Ok().content_type("image/jpeg").body(output)
}

#[get("/image/deliver")]
async fn image_deliver() -> impl Responder {
    println!("Delivering image");
    "sdfsd"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    println!("Starting server...");

    let images = Images::new();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(images))
            .service(image_request)
            .service(image_deliver)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
