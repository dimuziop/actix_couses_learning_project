use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::EzyTutorError;
use crate::models::course::{Course, CreateCourseDto, UpdateCourseDto};
use chrono::Utc;

pub async fn get_courses_by_tutor(pool: &PgPool, tutor_id: Uuid) -> Result<Vec<Course>, EzyTutorError> {
    let result = sqlx::query_as!(Course,
        r#"SELECT id,
                tutor_id,
                name,
                description,
                format,
                structure,
                duration,
                price,
                language,
                level,
                posted_time,
                created_at,
                updated_at,
                deleted_at
            FROM ezy_course_c4 WHERE tutor_id = $1 and deleted_at is null"#,
        tutor_id
    ).fetch_all(pool).await?;

    Ok(result)
}

pub async fn get_course(pool: &PgPool, tutor_id: Uuid, course_id: Uuid) -> Result<Course, EzyTutorError> {
    let course_row: Course = sqlx::query_as!(Course,
        r#"SELECT
                id,
                tutor_id,
                name,
                description,
                format,
                structure,
                duration,
                price,
                language,
                level,
                posted_time,
                created_at,
                updated_at,
                deleted_at
           FROM ezy_course_c4 WHERE tutor_id = $1 AND id = $2 and deleted_at is null"#,
        tutor_id, course_id,
    ).fetch_one(pool).await?;

    Ok(course_row)
}

pub async fn new_course(pool: &PgPool, course: CreateCourseDto) -> Result<Course, EzyTutorError> {
    let register_time = Utc::now().naive_utc();
    let inserted_course: Course = sqlx::query_as!(
        Course,
        r#"INSERT INTO
            ezy_course_c4 (
                id,
                tutor_id,
                name,
                description,
                format,
                structure,
                duration,
                price,
                language,
                level,
                posted_time,
                created_at,
                updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7,  $8, $9, $10, $11, $12, $13)
        returning
                id,
                tutor_id,
                name,
                description,
                format,
                structure,
                duration,
                price,
                language,
                level,
                posted_time,
                created_at,
                updated_at,
                deleted_at"#,
        Uuid::new_v4(),
        course.tutor_id,
        course.name,
        course.description,
        course.format,
        course.structure,
        course.duration,
        course.price,
        course.language,
        course.level,
        &register_time,
        &register_time,
        &register_time,
    ).fetch_one(pool).await?;

    Ok(inserted_course)
}


pub async fn update_course(pool: &PgPool, tutor_id: Uuid, course_id: Uuid, update_data: UpdateCourseDto) -> Result<Course, EzyTutorError> {
    let updated_course: Course = sqlx::query_as!(
        Course,
        r#"UPDATE ezy_course_c4
            SET name = $1,
                description = $2,
                format = $3,
                structure = $4,
                duration = $5,
                price = $6,
                language = $7,
                level = $8,
                updated_at = $9
        WHERE tutor_id = $10 AND id = $11 and deleted_at is null
        returning
                id,
                tutor_id,
                name,
                description,
                format,
                structure,
                duration,
                price,
                language,
                level,
                posted_time,
                created_at,
                updated_at,
                deleted_at"#,
        update_data.name,
        update_data.description,
        update_data.format,
        update_data.structure,
        update_data.duration,
        update_data.price,
        update_data.language,
        update_data.level,
        Utc::now().naive_utc(),
        tutor_id,
        course_id,
    ).fetch_one(pool).await?;

    Ok(updated_course)
}

pub async fn soft_delete_course(pool: &PgPool, tutor_id: Uuid, course_id: Uuid) -> Result<Course, EzyTutorError> {
    let deleted_course: Course = sqlx::query_as!(
        Course,
        r#"UPDATE ezy_course_c4
            SET deleted_at = $1
        WHERE tutor_id = $2 AND id = $3 and deleted_at is null
        returning
                id,
                tutor_id,
                name,
                description,
                format,
                structure,
                duration,
                price,
                language,
                level,
                posted_time,
                created_at,
                updated_at,
                deleted_at"#,
        Utc::now().naive_utc(),
        tutor_id,
        course_id,
    ).fetch_one(pool).await?;

    Ok(deleted_course)
}