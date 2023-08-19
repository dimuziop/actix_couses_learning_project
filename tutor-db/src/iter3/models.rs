use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub tutor_id: Uuid,
    pub course_id: Uuid,
    pub course_name: String,
    pub posted_time: NaiveDateTime,
}

impl From<web::Json<Course>> for Course {
    fn from(value: web::Json<Course>) -> Self {
        Course {
            tutor_id: value.tutor_id,
            course_id: value.course_id,
            course_name: value.course_name.clone(),
            posted_time: value.posted_time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct CourseDto {
    pub tutor_id: Uuid,
    pub course_name: String,
}

impl From<web::Json<CourseDto>> for CourseDto {
    fn from(value: web::Json<CourseDto>) -> Self {
        CourseDto {
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
        }
    }
}