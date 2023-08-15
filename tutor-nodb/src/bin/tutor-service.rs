use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use crate::state::AppState;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;
use routes::*;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good, you have asked already".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![])
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}