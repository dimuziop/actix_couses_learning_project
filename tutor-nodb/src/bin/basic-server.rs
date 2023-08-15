use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. Service Alive and Kicking")
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}