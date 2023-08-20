use actix_web::web;
use crate::handlers::{course::*, general::*};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses")
        .route("/", web::post().to(new_course))
        .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
        .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail))
        .route("/{tutor_id}/{course_id}", web::put().to(update_course_detail))
        .route("/{tutor_id}/{course_id}", web::delete().to(soft_delete_course))
    );
}