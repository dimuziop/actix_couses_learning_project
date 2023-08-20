use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::EzyTutorError;
use crate::models::course::Course;

pub async fn get_courses_by_tutor(pool: &PgPool, tutor_id: Uuid) -> Result<Vec<Course>, EzyTutorError> {
    let course_rows = sqlx::query!(
        r#"SELECT id, tutor_id, name, posted_time, created_at FROM ezy_course_c4 WHERE tutor_id = $1"#,
        tutor_id
    ).fetch_all(pool).await?;

    Ok(course_rows.iter().map(|course_row| Course {
        id: course_row.id,
        tutor_id: course_row.tutor_id,
        name: course_row.name.clone(),
        description: None,
        format: None,
        structure: None,
        duration: None,
        price: None,
        language: None,
        level: None,
        posted_time: chrono::NaiveDateTime::from(course_row.posted_time),
        created_at: chrono::NaiveDateTime::from(course_row.created_at),
        updated_at: None,
        deleted_at: None,
    }).collect())
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
           FROM ezy_course_c4 WHERE tutor_id = $1 AND id = $2"#,
        tutor_id, course_id,
    ).fetch_one(pool).await?;

    Ok(course_row)

    /*Ok(Course {
        id: course_row.id,
        tutor_id: course_row.tutor_id,
        name: course_row.name.clone(),
        description: None,
        format: None,
        structure: None,
        duration: None,
        price: None,
        language: None,
        level: None,
        posted_time: chrono::NaiveDateTime::from(course_row.posted_time),
        created_at: chrono::NaiveDateTime::from(course_row.created_at),
        updated_at: None,
        deleted_at: None,
    })*/
}

pub async fn new_course(pool: &PgPool, course: Course) -> Result<Course, EzyTutorError> {
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
                created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7,  $8, $9, $10, $11, $12)
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
        course.id,
        course.tutor_id,
        course.name,
        course.description,
        course.format,
        course.structure,
        course.duration,
        course.price,
        course.language,
        course.level,
        course.posted_time,
        course.created_at,
    ).fetch_one(pool).await?;

    Ok(inserted_course)
}