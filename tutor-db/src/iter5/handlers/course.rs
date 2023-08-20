use super::super::errors::EzyTutorError;
use super::super::
state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;
use crate::dbaccess::course;
use crate::models::course::{CreateCourseDto, UpdateCourseDto};

pub async fn new_course(course_dto: web::Json<CreateCourseDto>, app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    let dto: CreateCourseDto = course_dto.into();
    let course = course::new_course(&app_state.db, dto).await?;
    Ok(HttpResponse::Created().json(course))
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> Result<HttpResponse, EzyTutorError> {
    let courses = course::get_courses_by_tutor(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let course = course::get_course(&app_state.db, tutor_id, course_id).await?;
    Ok(HttpResponse::Ok().json(course))
}

pub async fn update_course_detail(app_state: web::Data<AppState>, course_dto: web::Json<UpdateCourseDto>, params: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let course = course::update_course(&app_state.db, tutor_id, course_id, course_dto.into()).await?;
    Ok(HttpResponse::Ok().json(course))
}

pub async fn soft_delete_course(app_state: web::Data<AppState>, params: web::Path<(Uuid, Uuid)>) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let course = course::soft_delete_course(&app_state.db, tutor_id, course_id).await?;
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
    use crate::models::course::Course;
    use serde_json::json;
    use super::*;

    pub fn init_test_debug() {
        match env_logger::try_init() {
            Ok(_) => {
                env::set_var("RUST_LOG", "debug");
                debug!("Debugger Up 👍");
            }
            Err(_) => {}
        }
    }

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreateCourseDto {
            tutor_id: Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(),
            name: "Some course name".into(),
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        });

        let resp = new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_empty() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let tutor_id: web::Path<Uuid> = web::Path::from(Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e74").unwrap());

        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        let actual_status = resp.status().clone();


        assert_eq!(actual_status, StatusCode::OK);

        let body = &resp.into_body().try_into_bytes().unwrap()[..];
        let actual = std::str::from_utf8(body).unwrap();
        let expected: Vec<Course> = vec![];
        assert_eq!(actual, json!(expected).to_string())
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_with_data() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let tutor_id: web::Path<Uuid> = web::Path::from(Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap());
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_not_found() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(), Uuid::from_str("70c57639-680a-44e8-a15b-e879d38aa854").unwrap()));
        let resp = get_course_detail(app_state, params).await;
        if resp.is_err() {
            assert_eq!(resp.unwrap_err().status_code(), StatusCode::NOT_FOUND)
        } else {
            assert_eq!(false, true)
        }
    }

    #[actix_rt::test]
    async fn get_course_detail_found() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(), Uuid::from_str("70c57639-680a-44e8-a15b-e879d38aa856").unwrap()));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreateCourseDto {
            tutor_id: Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(),
            name: "Some course name".into(),
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        });

        let resp = new_course(course, app_state.clone()).await.unwrap();
        let actual_status = resp.status().clone();
        let body = &resp.into_body().try_into_bytes().unwrap()[..];
        let actual = std::str::from_utf8(body).unwrap();
        let course: Course = serde_json::from_str::<Course>(actual).unwrap().into();
        assert_eq!(actual_status, StatusCode::CREATED);
        debug!("Created tutor_id: {} | course_id: {}", course.tutor_id, course.id);

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((course.tutor_id, course.id));

        let update_course = web::Json(UpdateCourseDto {
            name: "Some course name updated by tests".into(),
            description: Some("Some course name updated by tests".into()),
            format: Some("Some test format".into()),
            structure: Some("Test struct".into()),
            duration: Some("2 days".into()),
            price: Some(55),
            language: Some("test lang".into()),
            level: Some("beginners".into()),
        });


        let resp = update_course_detail(app_state.clone(), update_course, params).await.unwrap();
        let actual_status = resp.status().clone();
        let body = &resp.into_body().try_into_bytes().unwrap()[..];
        let actual = std::str::from_utf8(body).unwrap();
        let actual_course: Course = serde_json::from_str::<Course>(actual).unwrap().into();

        assert_eq!(actual_status, StatusCode::OK);
        debug!("Updated tutor_id: {} | course_id: {}", course.tutor_id, course.id);

        let expected_course = Course {
            id: course.id,
            tutor_id: course.tutor_id,
            name: "Some course name updated by tests".into(),
            description: Some("Some course name updated by tests".into()),
            format: Some("Some test format".into()),
            structure: Some("Test struct".into()),
            duration: Some("2 days".into()),
            price: Some(55),
            language: Some("test lang".into()),
            level: Some("beginners".into()),
            posted_time: actual_course.posted_time.clone(),
            created_at: actual_course.created_at.clone(),
            updated_at: actual_course.updated_at.clone(),
            deleted_at: None,
        };

        assert_eq!(expected_course.clone(), actual_course);

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((course.tutor_id, course.id));
        let resp = get_course_detail(app_state.clone(), params).await.unwrap();
        let actual_status = resp.status().clone();
        let body = &resp.into_body().try_into_bytes().unwrap()[..];
        let actual = std::str::from_utf8(body).unwrap();
        let actual_course: Course = serde_json::from_str::<Course>(actual).unwrap().into();

        assert_eq!(actual_status, StatusCode::OK);
        debug!("Get updated tutor_id: {} | course_id: {}", course.tutor_id, course.id);
        assert_eq!(expected_course.clone(), actual_course);
    }

    #[actix_rt::test]
    async fn update_course_failed_due_to_wrong_course_id() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e55").unwrap(), Uuid::from_str("70c57639-680a-44e8-a15b-e879d38aa886").unwrap()));
        let update_course = web::Json(UpdateCourseDto {
            name: "Some course name updated by tests".into(),
            description: Some("Some course name updated by tests".into()),
            format: Some("Some test format".into()),
            structure: Some("Test struct".into()),
            duration: Some("2 days".into()),
            price: Some(55),
            language: Some("test lang".into()),
            level: Some("beginners".into()),
        });


        let resp = update_course_detail(app_state.clone(), update_course, params).await;
        if resp.is_err() {
            assert_eq!(resp.unwrap_err().status_code(), StatusCode::NOT_FOUND)
        } else {
            assert_eq!(false, true)
        }
    }

    #[actix_rt::test]
    async fn soft_delete_course_success() {
        dotenv().ok();
        init_test_debug();

        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreateCourseDto {
            tutor_id: Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e61").unwrap(),
            name: "Some course name".into(),
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        });

        let resp = new_course(course, app_state.clone()).await.unwrap();
        let actual_status = resp.status().clone();
        let body = &resp.into_body().try_into_bytes().unwrap()[..];
        let actual = std::str::from_utf8(body).unwrap();
        let course: Course = serde_json::from_str::<Course>(actual).unwrap().into();
        assert_eq!(actual_status, StatusCode::CREATED);
        debug!("Created tutor_id: {} | course_id: {}", course.tutor_id, course.id);

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((course.tutor_id, course.id));
        let resp = soft_delete_course(app_state.clone(), params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        debug!("Deleted tutor_id: {} | course_id: {}", course.tutor_id, course.id);

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((course.tutor_id, course.id));
        let resp = get_course_detail(app_state.clone(), params).await;
        debug!("Already deleted tutor_id: {} | course_id: {}", course.tutor_id, course.id);
        if resp.is_err() {
            assert_eq!(resp.unwrap_err().status_code(), StatusCode::NOT_FOUND)
        } else {
            assert_eq!(false, true)
        }
    }

    #[actix_rt::test]
    async fn soft_delete_course_failed_due_to_wrong_course_id() {
        dotenv().ok();
        init_test_debug();
        let database_url = env::var("DATABASE_URL").expect("Database url is not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(Uuid, Uuid)> = web::Path::from((Uuid::from_str("d709c2c9-eeb8-4b6b-a63d-25ef38c78e55").unwrap(), Uuid::from_str("70c57639-680a-44e8-a15b-e879d38aa886").unwrap()));
        let resp = soft_delete_course(app_state.clone(), params).await;
        if resp.is_err() {
            assert_eq!(resp.unwrap_err().status_code(), StatusCode::NOT_FOUND)
        } else {
            assert_eq!(false, true)
        }
    }
}