use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::error::UnsupportedErrorKind::Color;
use image::{ColorType, DynamicImage, ExtendedColorType};
use jpeg_encoder::{Encoder, JpegColorType};
use serde::Deserialize;
use std::io::BufRead;
use std::thread;
use std::time::Instant;

#[derive(Deserialize, Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

struct Images {
    small: DynamicImage,
    medium: DynamicImage,
    large: DynamicImage,
}

impl Images {
    fn new() -> &'static Images {
        let start = Instant::now();

        let paths = vec![
            "src/files/image_small.JPG",
            "src/files/image_medium.JPG",
            "src/files/image_large.JPG",
        ];

        let images = thread::spawn(move || {
            let mut images = Vec::new();
            for path in paths {
                images.push(image::open(path).unwrap_or_default());
            }
            images
        })
        .join()
        .unwrap();

        let duration = start.elapsed();
        println!("Images loaded in: {:?}", duration);

        Box::leak(Box::new(Images {
            small: images[0].clone(),
            medium: images[1].clone(),
            large: images[2].clone(),
        }))
    }
}

#[get("/image/request/{size}")]
async fn image_request(size: web::Path<Size>, data: Data<&Images>) -> impl Responder {
    let mut output = Vec::new();
    let encoder = Encoder::new(&mut output, 96);

    encoder
        .encode(
            data.small.as_bytes(),
            data.small.width() as u16,
            data.small.height() as u16,
            jpeg_encoder::ColorType::Rgb,
        )
        .unwrap();

    HttpResponse::Ok().content_type("image/jpeg").body(output)

    // match size.into_inner() {
    //     Size::Small => HttpResponse::Ok()
    //         .content_type("image/jpeg")
    //         .body(data.small.as_bytes().to_vec()),
    //     Size::Medium => HttpResponse::Ok()
    //         .content_type("image/jpeg")
    //         .body(data.medium.as_bytes().to_vec()),
    //     Size::Large => HttpResponse::Ok()
    //         .content_type("image/jpeg")
    //         .body(data.large.as_bytes().to_vec()),
    // }
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
