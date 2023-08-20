use actix_web::web;
use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow, PartialOrd, PartialEq)]
pub struct Tutor {
    pub id: Uuid,
    pub name: String,
    pub pic_url: String,
    pub profile: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTutorDto {
    pub name: String,
    pub pic_url: String,
    pub profile: String,
}

impl From<web::Json<CreateTutorDto>> for CreateTutorDto {
    fn from(value: Json<CreateTutorDto>) -> Self {
        CreateTutorDto {
            name: value.name.clone(),
            pic_url: value.pic_url.clone(),
            profile: value.profile.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PatchTutorDto {
    pub name: Option<String>,
    pub pic_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<PatchTutorDto>> for PatchTutorDto {
    fn from(value: Json<PatchTutorDto>) -> Self {
        PatchTutorDto {
            name: value.name.clone(),
            pic_url: value.pic_url.clone(),
            profile: value.profile.clone(),
        }
    }
}