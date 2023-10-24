use crate::{Images, Size};
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};

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
