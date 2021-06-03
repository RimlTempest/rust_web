use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HoloQuery {
    id: usize
}

pub async fn get_holo_member(query: web::Query<HoloQuery>) -> impl Responder {
    // mutをつけないことによって不変の変数になる
    let user_name: Vec<&str> = vec!["潤羽るしあ", "宝鐘マリン", "兎田ぺこら", "不知火フレア", "白銀のえる"];
    let index = query.id;
    let name = user_name[index];
    HttpResponse::Ok().body(format!("id: {}\nname: {}", query.id, name))
}