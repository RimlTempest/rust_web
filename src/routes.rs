use actix_web::{web};
use crate::controllers::index;
use crate::controllers::get_user_id;
use crate::controllers::get_group_id;
use crate::controllers::get_holo_member;
use crate::controllers::post_holo_member;
use crate::controllers::post_project;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/users/{user_id}", web::get().to(get_user_id::get_user_id));
    cfg.route("/group/{group_id}/users/{user_id}", web::get().to(get_group_id::get_group_id));
    cfg.route("/holo", web::get().to(get_holo_member::post_holo_member));
    cfg.route("/holo", web::post().to(post_holo_member::get_holo_member));
    cfg.route("/project", web::post().to(post_project::post_project));
}