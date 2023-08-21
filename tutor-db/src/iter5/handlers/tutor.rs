use super::super::errors::EzyTutorError;
use super::super::
state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;
use crate::models::tutor::{CreateTutorDto, PatchTutorDto};
use crate::services;

pub async fn new_tutor(app_state: web::Data<AppState>, course_dto: web::Json<CreateTutorDto>) -> Result<HttpResponse, EzyTutorError> {
    services::tutor_service::create_tutor(app_state, course_dto.into()).await
        .map(|tutor| HttpResponse::Created().json(tutor))
}

pub async fn get_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    services::tutor_service::get_tutors(app_state).await.map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_by_id(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> Result<HttpResponse, EzyTutorError> {
    services::tutor_service::get_by_id(app_state, params.into_inner()).await.map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn update_tutor_detail(app_state: web::Data<AppState>, course_dto: web::Json<CreateTutorDto>, params: web::Path<Uuid>) -> Result<HttpResponse, EzyTutorError> {
    services::tutor_service::update(app_state, course_dto.into(), params.into_inner()).await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn partially_update_tutor_detail(app_state: web::Data<AppState>, course_dto: web::Json<PatchTutorDto>, params: web::Path<Uuid>) -> Result<HttpResponse, EzyTutorError> {
    services::tutor_service::partial_update(app_state, course_dto.into(), params.into_inner()).await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn soft_delete_tutor(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> Result<HttpResponse, EzyTutorError> {
    services::tutor_service::soft_delete(app_state, params.into_inner()).await.map(|tutors| HttpResponse::Ok().json(tutors))
}