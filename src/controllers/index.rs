use actix_web::{ HttpResponse, Responder};
use std::process::Command;

pub async fn index() -> impl Responder {
    let proc = Command::new("wmic")
        .args(&["os", "get", "caption"])
        .output()
        .expect("Failed to start `wmic`");
    // OSのバージョンを返す
    HttpResponse::Ok().body(format!("{}", String::from_utf8_lossy(&proc.stdout)))
}