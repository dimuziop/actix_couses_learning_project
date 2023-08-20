use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::EzyTutorError;
use crate::models::course::Course;

pub async fn get_courses_by_tutor(pool: &PgPool, tutor_id: Uuid) -> Result<Vec<Course>, EzyTutorError> {
    let course_rows = sqlx::query!(
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1"#,
        tutor_id
    ).fetch_all(pool).await?;

    Ok(course_rows.iter().map(|course_row| Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: chrono::NaiveDateTime::from(course_row.posted_time),
    }).collect())
}

pub async fn get_course(pool: &PgPool, tutor_id: Uuid, course_id: Uuid) -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query_as!(Course,
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1 AND course_id = $2"#,
        tutor_id, course_id,
    ).fetch_one(pool).await?;

    Ok(Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: chrono::NaiveDateTime::from(course_row.posted_time),
    })
}

pub async fn new_course(pool: &PgPool, course: Course) -> Result<Course, EzyTutorError> {
    let inserted_course: Course = sqlx::query_as!(
        Course,
        r#"INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name, posted_time) VALUES ($1, $2, $3, $4)
        returning course_id, tutor_id, course_name, posted_time"#,
        course.course_id, course.tutor_id, course.course_name, course.posted_time,
    ).fetch_one(pool).await?;

    Ok(inserted_course)
}