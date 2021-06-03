use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct HoloBody {
    id: String,
    name: String
}

pub async fn post_holo_member(data: web::Json<HoloBody>) -> impl Responder {
    HttpResponse::Ok().body(format!("id: {}\n name: {}", data.id, data.name))
}