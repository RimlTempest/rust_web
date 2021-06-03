use actix_web::{web, HttpResponse, Responder};

pub async fn get_group_id(web::Path((group_id, user_id)): web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("groupId: {} \nuserId: {}", group_id, user_id))
}