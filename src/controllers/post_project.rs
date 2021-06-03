use std::future::{Ready, ready};
use actix_web::{HttpRequest, HttpResponse, Responder, Error, web};
use serde::{Deserialize, Serialize};

// Projectモデル
#[derive(Deserialize)]
pub struct Project {
    project_id: i32,
    name: String,
    summary: String
}

// Projectのレスポンスモデル
#[derive(Serialize)]
pub struct ProjectResponse {
    project_id: i32,
    name: String,
    summary: String
}

// レスポンスモデルはResponderを継承
impl Responder for ProjectResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

// レスポンスをモデルをもとに取得
pub async fn post_project(data: web::Json<Project>) -> impl Responder {
    ProjectResponse { 
        project_id: data.project_id, 
        name: data.name.clone(), 
        summary: data.summary.clone()
    }
}