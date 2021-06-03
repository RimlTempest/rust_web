use actix_web::{web, HttpResponse, Responder};

pub async fn get_user_id(web::Path(user_id): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("user_id is {}", user_id))
}