use std::{env, io};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use log::debug;
use sqlx::PgPool;
use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug)]
pub struct Course {
    pub tutor_id: Uuid,
    pub course_id: Option<Uuid>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("Database url is not set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let course_rows = sqlx::query!(
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1"#, Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap()
    ).fetch_all(&db_pool).await.unwrap();

    let mut courses_list = vec![];
    for course_row in course_rows {
        courses_list.push(Course{
            course_id: Some(course_row.course_id),
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap()))
        })
    }
    debug!("Courses: {:?}", courses_list);
    Ok(())
}