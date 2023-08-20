use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow, PartialOrd, PartialEq)]
pub struct Course {
    pub id: Uuid,
    pub tutor_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
    pub posted_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(value: web::Json<Course>) -> Self {
        Course {
            id: value.id,
            tutor_id: value.tutor_id,
            name: value.name.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            structure: value.structure.clone(),
            duration: value.duration.clone(),
            price: value.price,
            language: value.language.clone(),
            level: value.level.clone(),
            posted_time: value.posted_time,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateCourseDto {
    pub tutor_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<CreateCourseDto>> for CreateCourseDto {
    fn from(value: web::Json<CreateCourseDto>) -> Self {
        CreateCourseDto {
            tutor_id: value.tutor_id,
            name: value.name.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            structure: value.structure.clone(),
            duration: value.duration.clone(),
            price: value.price,
            language: value.language.clone(),
            level: value.level.clone(),
        }
    }
}

/*impl TryFrom<web::Json<CreateCourseDto>> for CreateCourseDto {
    type Error = EzyTutorError;

    fn try_from(value: web::Json<CreateCourseDto>) -> Result<Self, Self::Error> {
        Ok(CreateCourseDto {
            tutor_id: value.tutor_id,
            name: value.name.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            structure: value.structure.clone(),
            duration: value.duration.clone(),
            price: value.price,
            language: value.language.clone(),
            level: value.level.clone(),
        })
    }
}*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateCourseDto {
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourseDto>> for UpdateCourseDto {
    fn from(value: web::Json<UpdateCourseDto>) -> Self {
        UpdateCourseDto {
            name: value.name.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            structure: value.structure.clone(),
            duration: value.duration.clone(),
            price: value.price,
            language: value.language.clone(),
            level: value.level.clone(),
        }
    }
}