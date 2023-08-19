use sqlx::PgPool;
use uuid::Uuid;
use crate::models::Course;

pub async fn get_courses_by_tutor(pool: &PgPool, tutor_id: Uuid) -> Vec<Course> {
    let course_rows = sqlx::query!(
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1"#,
        tutor_id
    ).fetch_all(pool).await.unwrap();

    course_rows.iter().map(|course_row| Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: chrono::NaiveDateTime::from(course_row.posted_time),
    }).collect()
}

pub async fn get_course(pool: &PgPool, tutor_id: Uuid, course_id: Uuid) -> Course {
    let course_row = sqlx::query!(
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1 AND course_id = $2"#,
        tutor_id, course_id,
    ).fetch_one(pool).await.unwrap();

    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: chrono::NaiveDateTime::from(course_row.posted_time),
    }
}

pub async fn new_course(pool: &PgPool, course: Course) -> Course {
    let inserted_course: Course = sqlx::query_as!(
        Course,
        r#"INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name, posted_time) VALUES ($1, $2, $3, $4)
        returning course_id, tutor_id, course_name, posted_time"#,
        course.course_id, course.tutor_id, course.course_name, course.posted_time,
    ).fetch_one(pool).await.unwrap();

    inserted_course
}