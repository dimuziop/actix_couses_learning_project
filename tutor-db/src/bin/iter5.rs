use std::{env, io};
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::{debug, error};
use sqlx::PgPool;
use crate::routes::{course_routes, general_routes};
use crate::state::AppState;

#[path = "../iter5/handlers/mod.rs"]
mod handlers;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
mod state;
#[path = "../iter5/models/mod.rs"]
mod models;
#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;

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
            .configure(course_routes)
            .service(web::scope("/api/v1")
                .configure(course_routes)
            )
    };

    match HttpServer::new(app).bind("127.0.0.1:3000")?.run().await {
        Ok(_) => {
            debug!("👍");
            Ok(())
        }
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    }
}