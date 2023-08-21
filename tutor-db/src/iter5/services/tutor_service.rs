use actix_web::web;
use uuid::Uuid;
use crate::dbaccess;
use crate::errors::EzyTutorError;
use crate::models::tutor::{CreateTutorDto, PatchTutorDto, Tutor};
use crate::state::AppState;

// Try to remove web dependencies here, should be framework agnostic
pub async fn get_tutors(app_state: web::Data<AppState>) -> Result<Vec<Tutor>, EzyTutorError>{
    dbaccess::tutor::get_tutors(&app_state.db).await
}

pub async fn get_by_id(app_state: web::Data<AppState>, tutor_id: Uuid) -> Result<Tutor, EzyTutorError>{
    dbaccess::tutor::by_id(&app_state.db, tutor_id).await
}

pub async fn create_tutor(app_state: web::Data<AppState>, tutor_dto: CreateTutorDto) -> Result<Tutor, EzyTutorError>{
    dbaccess::tutor::create(&app_state.db, tutor_dto.clone()).await
}

pub async fn update(app_state: web::Data<AppState>, tutor_dto: CreateTutorDto, tutor_id: Uuid) -> Result<Tutor, EzyTutorError>{
    dbaccess::tutor::update(&app_state.db, tutor_dto, tutor_id).await
}

pub async fn partial_update(app_state: web::Data<AppState>, tutor_dto: PatchTutorDto, tutor_id: Uuid) -> Result<Tutor, EzyTutorError>{
    let due_tutor = dbaccess::tutor::by_id(&app_state.db, tutor_id).await?;
    let updated_tutor = CreateTutorDto {
        name: tutor_dto.name.unwrap_or(due_tutor.name.clone()),
        pic_url: tutor_dto.pic_url.unwrap_or(due_tutor.pic_url.clone()),
        profile: tutor_dto.profile.unwrap_or(due_tutor.profile.clone()),
    };
    dbaccess::tutor::update(&app_state.db, updated_tutor, tutor_id).await
}

pub async fn soft_delete(app_state: web::Data<AppState>, tutor_id: Uuid) -> Result<Tutor, EzyTutorError>{
    dbaccess::tutor::soft_delete(&app_state.db, tutor_id).await
}

