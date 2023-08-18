use std::{env, io};
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::debug;
use sqlx::PgPool;
use crate::routes::{course_routes, general_routes};
use crate::state::AppState;

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;
#[path = "../iter2/models.rs"]
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("Database url is not set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good, you have asked already".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .service(web::scope("/v1")
                .configure(course_routes)
            )
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await;

    Ok(())


}