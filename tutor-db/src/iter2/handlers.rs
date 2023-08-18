use actix_web::{HttpResponse, web};
use uuid::Uuid;
use crate::models::Course;
use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    log::debug!("Received new course");

    HttpResponse::Created().json("All happy")
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json("All happy")
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(Uuid, Uuid)>) -> HttpResponse {
    HttpResponse::Ok().json("All happy")
}

#[cfg(test)]
mod test {
    use std::env;
    use std::str::FromStr;
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use super::*;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state:  web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let course = web::Json(Course {
            tutor_id: Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(),
            course_name: "Some course name".into(),
            course_id: None,
            posted_time: None,
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_empty() {
        assert_eq!(true, true)
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_with_data() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state:  web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let tutor_id: web::Path<Uuid> = web::Path::from( Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap());
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_not_found() {
        assert_eq!(true, true)
    }

    #[actix_rt::test]
    async fn get_course_detail_found() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state:  web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let params: web::Path<(Uuid, Uuid)> = web::Path::from( (Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(), Uuid::from_str("9b6a04e9-15d0-4f27-8b09-60485ac9f99f").unwrap()));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}