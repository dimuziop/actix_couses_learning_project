use actix_web::{HttpResponse, web};
use chrono::Utc;
use uuid::Uuid;
use crate::models::{Course, CourseDto};
use crate::state::AppState;
use crate::db_access;
use crate::errors::EzyTutorError;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(course_dto: web::Json<CourseDto>, app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    let dto: CourseDto = course_dto.into();
    let insert_course = Course {
        course_id: Uuid::new_v4(),
        tutor_id: dto.tutor_id,
        course_name: dto.course_name.clone(),
        posted_time: Utc::now().naive_utc(),

    };
    let course = db_access::new_course(&app_state.db, insert_course).await?;
    Ok(HttpResponse::Created().json(course))
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> Result<HttpResponse, EzyTutorError> {
    let courses = db_access::get_courses_by_tutor(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let course = db_access::get_course(&app_state.db, tutor_id, course_id).await?;
   Ok(HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod test {
    use std::env;
    use std::str::FromStr;
    use std::sync::Mutex;
    use actix_web::body::MessageBody;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use log::debug;
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

        let course = web::Json(CourseDto {
            tutor_id: Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(),
            course_name: "Some course name".into(),
        });

        let resp = new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_empty() {
//TODO
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
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_not_found() {

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state:  web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });

        let params: web::Path<(Uuid, Uuid)> = web::Path::from( (Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(), Uuid::from_str("70c57639-680a-44e8-a15b-e879d38aa854").unwrap()));
        let resp = get_course_detail(app_state, params).await;
        if resp.is_err() {
            assert_eq!(resp.unwrap_err().status_code(), StatusCode::NOT_FOUND)
        }
        else {
            assert_eq!(false, true)
        }
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

        let params: web::Path<(Uuid, Uuid)> = web::Path::from( (Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(), Uuid::from_str("70c57639-680a-44e8-a15b-e879d38aa856").unwrap()));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}