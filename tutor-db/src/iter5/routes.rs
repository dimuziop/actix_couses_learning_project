use actix_web::web;
use crate::handlers::{course::*, general::*, tutor::*};

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

pub fn tutor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tutors")
        .route("/", web::post().to(new_tutor))
        .route("/", web::get().to(get_tutors))
        .route("/{tutor_id}", web::get().to(get_tutor_by_id))
        .route("/{tutor_id}", web::put().to(update_tutor_detail))
        .route("/{tutor_id}", web::patch().to(partially_update_tutor_detail))
        .route("/{tutor_id}", web::delete().to(soft_delete_tutor))
    );
}