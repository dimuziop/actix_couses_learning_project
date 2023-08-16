use actix_web::{HttpResponse, web};
use chrono::Utc;
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

    //let course_count_for_user = app_state.courses.lock().unwrap().clone().into_iter().filter(|course| course.tutor_id == new_course.tutor_id).count();
    let course_id = Uuid::new_v4();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_id.clone()),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Created().json(format!("Created course: {:?}", course_id))
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<Uuid>) -> HttpResponse {
    let tutor_id = params.into_inner();
    let filter_courses = app_state.courses.lock().unwrap().clone().into_iter().filter(|course| course.tutor_id == tutor_id).collect::<Vec<Course>>();

    HttpResponse::Ok().json(filter_courses)
}

#[cfg(test)]
mod test {
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use super::*;
    use std::str::FromStr;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: Uuid::new_v4(),
            course_name: "Some course name".into(),
            course_id: None,
            posted_time: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::CREATED)
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_empty() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let tutor_id: web::Path<Uuid> = web::Path::from( Uuid::from_str("15d0830e-34d2-4914-873d-81725a5bc431").unwrap());
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_tutor_courses_with_data() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![Course {
                tutor_id: Uuid::from_str("15d0830e-34d2-4914-873d-81725a5bc431").unwrap(),
                course_id: Some(Uuid::new_v4()),
                course_name: "some name".to_string(),
                posted_time: Some(Utc::now().naive_utc()),
            }]),
        });

        let tutor_id: web::Path<Uuid> = web::Path::from( Uuid::from_str("15d0830e-34d2-4914-873d-81725a5bc431").unwrap());
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}