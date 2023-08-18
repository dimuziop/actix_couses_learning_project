use actix_web::{HttpResponse, web};
use uuid::Uuid;
use crate::models::Course;
use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    log::debug!("Received new course");

    HttpResponse::Ok().json("All happy")
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json("All happy")
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(Uuid, Uuid)>) -> HttpResponse {
    HttpResponse::Ok().json("All happy")
}