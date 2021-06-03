use actix_web::{ App, HttpServer };
use rust_web::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HTTPサーバを8080で起動
    HttpServer::new(|| App::new().configure(routes::routes))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

